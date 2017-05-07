use uuid::Uuid;
use errors::*;
use {
    QPanopticon,
};
use singleton::{
    VarName,
    AbstractInterpretation,
};
use panopticon::{
    Function,
    Kset,
    ControlFlowTarget,
    Lvalue,
    Rvalue,
    approximate,
    BasicBlock,
};
use graph_algos::{
    GraphTrait,
};
use std::collections::{
    HashMap,
    HashSet,
};
use std::iter::{
    FromIterator,
    IntoIterator,
};

#[derive(Clone)]
enum ActionPayload {
    Comment {
        address: u64,
        before: String,
        after: String,
    },
    Rename {
        before: String,
        after: String,
    },
    SetValue {
        variable: VarName,
        before: Option<AbstractInterpretation>,
        after: Option<AbstractInterpretation>,
        modified_basic_blocks: Vec<u64>,
    }
}

#[derive(Clone)]
pub struct Action {
    function: Uuid,
    payload: ActionPayload,
}

impl Action {
    fn new(f: Uuid,p: ActionPayload) -> Action {
        Action{
            function: f,
            payload: p,
        }
    }

    pub fn new_comment(panopticon: &mut QPanopticon,func: Uuid, address: u64, comment: String) -> Result<Action> {
        Ok(Self::new(func,ActionPayload::Comment{
            address: address,
            before: panopticon.control_flow_comments.get(&address).cloned().unwrap_or("".to_string()),
            after: comment,
        }))
    }

    pub fn new_rename(panopticon: &mut QPanopticon,func: Uuid, name: String) -> Result<Action> {
        Ok(Self::new(func,ActionPayload::Rename{
            before: panopticon.functions.get(&func).map(|f| f.name.clone()).unwrap_or("".to_string()),
            after: name,
        }))
    }

    pub fn new_setvalue(panopticon: &mut QPanopticon,func: Uuid, variable: VarName, value: Option<Kset>) -> Result<Action> {
        let before = panopticon.control_flow_values.get(&func);
        let mut input = before.map(|x| x.input.clone()).unwrap_or(HashMap::new());

        if let Some(ref value) = value {
            input.insert(variable.clone(),value.clone());
        } else {
            input.remove(&variable);
        }

        let after = if input.is_empty() {
            None
        } else {
            let output = {
                let i = input.iter().map(|(k,v)| ((k.name.clone(),k.subscript),v.clone()));
                let function = panopticon.functions.get(&func).unwrap();
                let fixed = HashMap::from_iter(i);

                approximate(function,&fixed)?
            };
            let o = output.into_iter().filter_map(|(k,v)| {
                match k {
                    Lvalue::Variable{ name, subscript: Some(subscript),.. } => Some((VarName{ name: name, subscript: subscript },v)),
                    Lvalue::Variable{ subscript: None,.. } => None,
                    Lvalue::Undefined{ .. } => None
                }
            });

            Some(AbstractInterpretation{
                input: input,
                output: HashMap::from_iter(o),
            })
        };

        let function = panopticon.functions.get(&func).unwrap();
        let addrs = diff_abstract_interpretations(after.as_ref(),before,&function);

        Ok(Self::new(func,ActionPayload::SetValue{
            variable: variable,
            before: before.cloned(),
            after: after,
            modified_basic_blocks: addrs,
        }))
    }

    pub fn undo(&self,panopticon: &mut QPanopticon) -> Result<()> {
        match self.payload {
            ActionPayload::Comment { address, ref before, ref after } => {
                debug_assert!(panopticon.control_flow_comments.get(&address).unwrap_or(&"".to_string()) == after);
                panopticon.control_flow_comments.insert(address,before.clone());
                panopticon.update_basic_block(&vec![address],&self.function)
            },
            ActionPayload::Rename{ ref before,.. } => {
                if let Some(func) = panopticon.functions.get_mut(&self.function) {
                    func.name = before.clone();
                }

                panopticon.update_sidebar(&self.function)

            },
            ActionPayload::SetValue{ ref before, ref modified_basic_blocks,.. } => {
                if let &Some(ref before) = before {
                    panopticon.control_flow_values.insert(self.function.clone(),before.clone());
                } else {
                    panopticon.control_flow_values.remove(&self.function);
                }
                panopticon.update_basic_block(modified_basic_blocks,&self.function)
            }
        }
    }

    pub fn redo(&self,panopticon: &mut QPanopticon) -> Result<()> {
        match self.payload {
            ActionPayload::Comment { address, ref before, ref after } => {
                debug_assert!(panopticon.control_flow_comments.get(&address).unwrap_or(&"".to_string()) == before);
                panopticon.control_flow_comments.insert(address,after.clone());
                panopticon.update_basic_block(&vec![address],&self.function)
            },
            ActionPayload::Rename{ ref after,.. } => {
                if let Some(func) = panopticon.functions.get_mut(&self.function) {
                    func.name = after.clone();
                }

                panopticon.update_sidebar(&self.function)
            },
            ActionPayload::SetValue{ ref after, ref modified_basic_blocks,.. } => {
                if let &Some(ref after) = after {
                    panopticon.control_flow_values.insert(self.function.clone(),after.clone());
                } else {
                    panopticon.control_flow_values.remove(&self.function);
                }
                panopticon.update_basic_block(modified_basic_blocks,&self.function)
            }
        }
    }
}

fn diff_abstract_interpretations(a: Option<&AbstractInterpretation>, b: Option<&AbstractInterpretation>, func: &Function) -> Vec<u64> {
    let a = if let Some(a) = a { HashSet::<(&VarName,&Kset)>::from_iter(a.output.iter()) } else { HashSet::new() };
    let b = if let Some(b) = b { HashSet::<(&VarName,&Kset)>::from_iter(b.output.iter()) } else { HashSet::new() };
    let blocks = func.postorder().into_iter().filter_map(|vx| {
        if let Some(&ControlFlowTarget::Resolved(ref bb)) = func.cflow_graph.vertex_label(vx) {
            Some(bb)
        } else {
            None
        }
    }).collect::<Vec<&BasicBlock>>();

    let mut ret = vec![];
    let nams = HashSet::<VarName>::from_iter(a.symmetric_difference(&b).map(|&(n,_)| n.clone()));

    for bb in blocks.iter() {
        let mut hit = false;

        bb.execute(|stmt| {
            if let Lvalue::Variable{ ref name, subscript: Some(subscript),.. } = stmt.assignee {
                hit |= nams.contains(&VarName{ name: name.clone(), subscript: subscript });
            }

            hit |= stmt.op.operands().iter().any(|rv| {
                if let &&Rvalue::Variable{ ref name, subscript: Some(subscript),.. } = rv {
                    nams.contains(&VarName{ name: name.clone(), subscript: subscript })
                } else {
                    false
                }
            });
        });

        if hit {
            ret.push(bb.area.start);
        }
    }

    ret.sort();
    ret.dedup();

    ret
}
