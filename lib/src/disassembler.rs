/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2014-2015 Kai Michaelis
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

#![macro_use]

use std::rc::Rc;
use std::fmt::Debug;
use std::ops::{BitAnd,BitOr,Shl,Shr,Not};
use std::collections::{HashSet,HashMap};
use std::mem::size_of;

use num::traits::{Zero,One,NumCast,cast};
use graph_algos::{
    AdjacencyList,
    GraphTrait,
    MutableGraphTrait,
    IncidenceGraphTrait,
    VertexListGraphTrait,
    EdgeListGraphTrait
};
use graph_algos::adjacency_list::{
    AdjacencyListVertexDescriptor
};

use value::Rvalue;
use mnemonic::Mnemonic;
use guard::Guard;

use codegen::CodeGen;
use layer::LayerIter;

pub trait Architecture: Clone
{
    type Token: Not<Output=Self::Token> +
                Clone +
                Zero +
                One +
                Debug +
                NumCast +
                BitOr<Output=Self::Token> +
                BitAnd<Output=Self::Token> +
                Shl<usize,Output=Self::Token> +
                Shr<usize,Output=Self::Token> +
                PartialEq +
                Eq;
    type Configuration: Clone;
}

pub type Action<A> = fn(&mut State<A>) -> bool;

#[derive(Debug,Clone)]
pub struct State<A: Architecture> {
    // in
    pub address: u64,
    pub tokens: Vec<A::Token>,
    pub groups: Vec<(String,u64)>,

    // out
    pub mnemonics: Vec<Mnemonic>,
    pub jumps: Vec<(u64,Rvalue,Guard)>,

    mnemonic_origin: u64,
    jump_origin: u64,
    pub configuration: A::Configuration,
}

impl<A: Architecture> State<A> {
    pub fn new(a: u64,c: A::Configuration) -> State<A> {
        State{
            address: a,
            tokens: vec!(),
            groups: vec!(),
            mnemonics: Vec::new(),
            jumps: Vec::new(),
            mnemonic_origin: a,
            jump_origin: a,
            configuration: c,
        }
    }

    pub fn get_group(&self,n: &str) -> u64 {
        self.groups.iter().find(|x| x.0 == n.to_string()).unwrap().1.clone()
    }

    pub fn has_group(&self,n: &str) -> bool {
        self.groups.iter().find(|x| x.0 == n.to_string()).is_some()
    }

    pub fn mnemonic<'a,F: Fn(&mut CodeGen<A>) -> ()>(&mut self,len: usize, n: &str, fmt: &str, ops: Vec<Rvalue>, f: &F) {
        self.mnemonic_dynargs(len,n,fmt,&|cg: &mut CodeGen<A>| -> Vec<Rvalue> {
            f(cg);
            ops.clone()
        });
    }

    pub fn mnemonic_dynargs<F>(&mut self,len: usize, n: &str, fmt: &str, f: &F)
    where F: Fn(&mut CodeGen<A>) -> Vec<Rvalue> {
        let mut cg = CodeGen::new(&self.configuration);
        let ops = f(&mut cg);

        self.configuration = cg.configuration;
        self.mnemonics.push(Mnemonic::new(
                self.mnemonic_origin..(self.mnemonic_origin + (len as u64)),
                n.to_string(),
                fmt.to_string(),
                ops.iter(),
                cg.instructions.iter()));
        self.jump_origin = self.mnemonic_origin;
        self.mnemonic_origin += len as u64;
    }

    pub fn jump(&mut self,v: Rvalue,g: Guard) {
        let o = self.jump_origin;
        self.jump_from(o,v,g);
    }

    pub fn jump_from(&mut self,origin: u64,v: Rvalue,g: Guard) {
        self.jumps.push((origin,v,g));
    }
}

#[derive(Clone)]
pub enum Expr<A: Architecture> {
    Pattern(String),
    Terminal(A::Token),
    Subdecoder(Rc<Disassembler<A>>),
    Optional(Box<Expr<A>>),
}

pub trait ToExpr<A: Architecture> {
    fn to_expr(&self) -> Expr<A>;
}

impl<'a,A: Architecture> ToExpr<A> for &'a str {
    fn to_expr(&self) -> Expr<A> {
        Expr::Pattern(self.to_string())
    }
}

impl<'a,A: Architecture> ToExpr<A> for Rc<Disassembler<A>> {
    fn to_expr(&self) -> Expr<A> {
        Expr::Subdecoder(self.clone())
    }
}

impl<A: Architecture> ToExpr<A> for usize {
    fn to_expr(&self) -> Expr<A> {
        Expr::Terminal(<A::Token as NumCast>::from::<usize>(*self).unwrap().clone())
    }
}

impl<A: Architecture> ToExpr<A> for Expr<A> {
    fn to_expr(&self) -> Expr<A> {
        self.clone()
    }
}

#[derive(Clone)]
pub enum Match<A: Architecture> {
    Epsilon,
    Character{
        bits: A::Token,
        mask: A::Token,
        capture: Vec<(String,A::Token)>,
    },
    Condition(Rc<Action<A>>),
}

pub struct Disassembler<A: Architecture> {
    pub start: AdjacencyListVertexDescriptor,
    pub goals: HashSet<AdjacencyListVertexDescriptor>,
    pub graph: AdjacencyList<usize,Match<A>>,
    pub default: Option<Action<A>>,
    pub next_index: usize,
}

impl<A: Architecture> Disassembler<A> {
    pub fn new() -> Disassembler<A> {
        let mut g = AdjacencyList::<usize,Match<A>>::new();

        Disassembler::<A> {
            start: g.add_vertex(0),
            graph: g,
            goals: HashSet::<AdjacencyListVertexDescriptor>::new(),
            default: None,
            next_index: 1,
        }
    }

    pub fn set_default(&mut self,f: Action<A>) {
        self.default = Some(f);
    }

    fn append_expr(&mut self, prev: AdjacencyListVertexDescriptor, expr: Expr<A>, index: usize) -> AdjacencyListVertexDescriptor {
        match expr {
            Expr::Pattern(ref s) => {
                let mut groups = HashMap::<String,A::Token>::new();
                let mut cur_group = "".to_string();
                let mut read_pat = false; // false while reading torwards @
                let mut bit: isize = (size_of::<A::Token>() * 8) as isize;
                let mut mask = A::Token::zero();
                let mut pat = A::Token::zero();

                for c in s.chars() {
                    match c {
                        '@' => {
                            if read_pat {
                                panic!("Pattern syntax error: read '@' w/o name in '{}'",s);
                            } else {
                                read_pat = true;

                                if cur_group == "" {
                                    panic!("Pattern syntax error: anonymous groups not allowed in '{}'",s);
                                }

                                groups.insert(cur_group.clone(),A::Token::zero());
                            }
                        },
                        ' ' => {
                            read_pat = false;
                            cur_group = "".to_string();
                        },
                        '.' => {
                            if bit <= 0 {
                                panic!("too long bit pattern");
                            }

                            if read_pat && cur_group != "" {
                                *groups.get_mut(&cur_group).unwrap() = groups.get(&cur_group).unwrap().clone() | (A::Token::one() << ((bit - 1) as usize));
                            }

                            bit -= 1;
                        },
                        '0' | '1' => {
                            if bit <= 0 {
                                panic!("too long bit pattern");
                            }

                            if bit - 1 > 0 {
                                mask = mask | (A::Token::one() << ((bit - 1) as usize));
                            } else {
                                mask = mask | A::Token::one();
                            }

                            if c == '1' {
                                pat = pat | (A::Token::one() << ((bit - 1) as usize));
                            }

                            if read_pat && cur_group != "" {
                                *groups.get_mut(&cur_group).unwrap() = groups.get(&cur_group).unwrap().clone() | (A::Token::one() << ((bit - 1) as usize));
                            }

                            bit -= 1;
                        },
                        'a'...'z' | 'A'...'Z' => {
                            if read_pat {
                                panic!("Pattern syntax error: undelimited capture group name in '{}'",s);
                            } else {
                                cur_group.push(c);
                            }
                        },
                        _ => {
                            panic!("Pattern syntax error: invalid character '{}' in '{}'",c,s);
                        }
                    }
                }

                if bit != 0 {
                    panic!("Pattern syntax error: invalid pattern length in '{}'",s);
                }

                let next = self.graph.add_vertex(index);

                self.graph.add_edge(Match::<A>::Character{
                    bits: pat,
                    mask: mask,
                    capture: groups.iter().filter_map(|x| {
                        if *x.1 != A::Token::zero() {
                            Some((x.0.clone(),x.1.clone()))
                        } else {
                            None
                        }
                    }).collect(),
                },prev,next);

                next
            },
            Expr::Terminal(ref i) => {
                let next = self.graph.add_vertex(index);

                self.graph.add_edge(Match::<A>::Character{
                    bits: i.clone(),
                    mask: !A::Token::zero(),
                    capture: vec![],
                },prev,next);

                next
            },
            Expr::Subdecoder(ref m) => {
                let mut trans = HashMap::<AdjacencyListVertexDescriptor,AdjacencyListVertexDescriptor>::new();

                for vx in m.graph.vertices() {
                    trans.insert(vx,self.graph.add_vertex(index));
                }

                for ed in m.graph.edges() {
                    self.graph.add_edge(m.graph.edge_label(ed).unwrap().clone(),
                                        trans[&m.graph.source(ed)],
                                        trans[&m.graph.target(ed)]);
                }

                self.graph.add_edge(Match::Epsilon::<A>,prev,trans[&m.start]);

                let last = self.graph.add_vertex(index);

                for f in m.goals.iter() {
                    self.graph.add_edge(Match::Epsilon::<A>,trans[f],last);
                }

                last
            },
            Expr::Optional(ref e) => {
                let next = self.append_expr(prev,*e.clone(),index);
                let last = self.graph.add_vertex(index);
                self.graph.add_edge(Match::Epsilon::<A>,prev,last);
                self.graph.add_edge(Match::Epsilon::<A>,next,last);

                last
            },
        }
    }

    pub fn append_conjunction(&mut self, e: Vec<Expr<A>>, a: Action<A>) {
        let mut prev = None;
        let index = self.next_index;

        for expr in e {
            let s = self.start.clone();
            prev = Some(self.append_expr(prev.unwrap_or(s),expr,index));
        }

        if let Some(p) = prev {
            let last = self.graph.add_vertex(index);
            self.graph.add_edge(Match::Condition::<A>(Rc::new(a)),p,last);
            self.goals.insert(last);
        }

        self.next_index += 1;
    }

    pub fn to_dot(&self) {
        println!("digraph G {{");
        for v in self.graph.vertices() {
            let lb = self.graph.vertex_label(v).unwrap();

            if self.goals.contains(&v) {
                println!("{} [label=\"{}, prio: {}\",shape=doublecircle]",v.0,v.0,lb);
            } else {
                println!("{} [label=\"{}, prio: {}\",shape=circle]",v.0,v.0,lb);
            }
        }
        for e in self.graph.edges() {
            let lb = match self.graph.edge_label(e) {
                Some(&Match::Epsilon::<A>) => "*".to_string(),
                Some(&Match::Character::<A>{ ref bits, ref mask,.. }) => format!("{:?}/{:?}",bits,mask),
                Some(&Match::Condition::<A>(_)) => "f()".to_string(),
                None => "".to_string(),
            };
            println!("{} -> {} [label=\"{}\"]",self.graph.source(e).0,self.graph.target(e).0,lb);
        }
        println!("}}");
    }

    pub fn next_match(&self,i: &mut LayerIter, offset: u64, cfg: A::Configuration) -> Option<State<A>> {
        let mut states = HashMap::<AdjacencyListVertexDescriptor,State<A>>::new();
        let mut tokens = Vec::<A::Token>::new();
        let mut j = i.clone();
        let min_len = |len: usize, ts: &mut Vec<A::Token>, j: &mut LayerIter| -> bool {
            if ts.len() >= len {
                true
            } else {
                for _ in ts.len()..len {
                    let mut tmp: A::Token = A::Token::zero();

                    for x in (0..size_of::<A::Token>()) {
                        if let Some(Some(b)) = j.next() {
                            if x != 0 {
                                tmp = tmp | (cast::<u8,A::Token>(b).unwrap() << 8);
                            } else {
                                tmp = cast(b).unwrap();
                            }
                        } else {
                            return false;
                        }
                    }
                    ts.push(tmp);
                }

                (ts.len() >= len)
            }
        };

        states.insert(self.start,State::<A>::new(offset,cfg.clone()));

        loop {
            let mut next_states = HashMap::<AdjacencyListVertexDescriptor,State<A>>::new();
            for (&pos,state) in states.iter() {
                if self.goals.contains(&pos) {
                    assert!(self.graph.out_degree(pos) == 0);
                    next_states.insert(pos,state.clone());
                } else {
                    for e in self.graph.out_edges(pos) {
                        let m = self.graph.edge_label(e).unwrap();
                        let mut st = state.clone();
                        let has_match = match m {
                            &Match::Epsilon::<A> => true,
                            &Match::Character::<A>{ ref bits, ref mask, ref capture } => {
                                let l = st.tokens.len();
                                if min_len(l + 1,&mut tokens,&mut j) && bits.clone() == (tokens[l].clone() & mask.clone()) {
                                    let t = tokens[l].clone();
                                    st.tokens.push(t.clone());

                                    for &(ref name,ref mask) in capture.iter() {
                                        let mut res = if let Some(p) = st.groups.iter().position(|x| x.0 == *name) {
                                            st.groups[p].1
                                        } else {
                                            0u64
                                        };

                                        for rbit in (0..(size_of::<A::Token>() * 8)) {
                                            let bit = (size_of::<A::Token>() * 8) - rbit - 1;
                                            let bit_mask = if bit > 0 {
                                                A::Token::one() << bit
                                            } else {
                                                A::Token::one()
                                            };

                                            let a = bit_mask.clone() & mask.clone();

                                            if a != A::Token::zero() {
                                                res <<= 1;

                                                if t.clone() & a != A::Token::zero() {
                                                    res |= 1;
                                                }
                                            }
                                        }

                                        if let Some(p) = st.groups.iter().position(|x| x.0 == *name) {
                                            st.groups[p].1 = res;
                                        } else {
                                            st.groups.push((name.clone(),res));
                                        }
                                    }

                                    true
                                } else {
                                    false
                                }
                            },
                            &Match::Condition::<A>(ref a) => (*a)(&mut st),
                        };

                        if has_match {
                            next_states.insert(self.graph.target(e),st);
                        }
                    }
                }
            }

            if states.keys().collect::<HashSet<_>>() == next_states.keys().collect::<HashSet<_>>() {
                break;
            } else {
                states = next_states;
            }
        }

        if states.len() > 0 {
            let mut states_vec = states.iter().collect::<Vec<_>>();
            states_vec.sort_by(|a,b| self.graph.vertex_label(*a.0).unwrap().cmp(self.graph.vertex_label(*b.0).unwrap()));
            Some(states_vec[0].1.clone())
        } else {
            if self.default.is_some() && min_len(1,&mut tokens,&mut j) {
                let mut st = State::<A>::new(offset,cfg.clone());

                st.tokens = vec!(tokens.iter().next().unwrap().clone());

                if self.default.unwrap()(&mut st) {
                    Some(st)
                } else {
                    None
                }
            } else {
                None
            }
        }
    }
}

macro_rules! opt {
    ($e:expr) => { { Expr::Optional(Box::new($e.to_expr())) } };
}

#[macro_export]
macro_rules! new_disassembler {
    ($ty:ty => $( [ $( $t:expr ),+ ] = $f:expr),+) => {
        {
            let mut dis = ::disassembler::Disassembler::<$ty>::new();

            $({
                let mut __x = ::std::vec::Vec::new();
                $(
                    __x.push($t.to_expr());
                )+
                fn a(a: &mut ::disassembler::State<$ty>) -> bool { ($f)(a) };
                let fuc: ::disassembler::Action<$ty> = a;
                dis.append_conjunction(__x,fuc);
            })+

            ::std::rc::Rc::<::disassembler::Disassembler<$ty>>::new(dis)
        }
    };
    ($ty:ty => $( [ $( $t:expr ),+ ] = $f:expr),+, _ = $def:expr) => {
        {
            let mut dis = Disassembler::<>::new();

            $({
                let mut __x = Vec::new();
                $(
                    __x.push($t.to_expr());
                )+
                fn a(a: &mut State<$ty>) -> bool { ($f)(a) };
                let fuc: Action<$ty> = a;
                dis.append_conjunction(__x,fuc);
            })+

            fn __def(st: &mut State<$ty>) -> bool { ($def)(st) };
            dis.set_default(__def);

            ::std::rc::Rc::<Disassembler<$ty>>::new(dis)
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;
    use layer::OpaqueLayer;
    use guard::Guard;
    use value::Rvalue;
    use mnemonic::Bound;

    #[derive(Clone)]
    enum TestArchShort {}
    impl Architecture for TestArchShort {
        type Token = u8;
        type Configuration = ();
    }

    #[derive(Clone)]
    enum TestArchWide {}
    impl Architecture for TestArchWide {
        type Token = u16;
        type Configuration = ();
    }

    #[test]
    fn combine_expr() {
        let sub = new_disassembler!(TestArchShort =>
            [ 1 ] = &|_| { true },
            [ 2, 2 ] = &|_| { true }
        );

        let main = new_disassembler!(TestArchShort =>
            [ 3, sub ] = &|_| { true }
        );

        main.to_dot();
        let src = OpaqueLayer::wrap(vec!(3,1,3,2,2));

        {
            let maybe_res = main.next_match(&mut src.iter(),0,());

            assert!(maybe_res.is_some());
            let res = maybe_res.unwrap();

            assert_eq!(res.address, 0);
            assert_eq!(res.tokens.len(), 2);
            assert_eq!(res.tokens[0], 3);
            assert_eq!(res.tokens[1], 1);
            assert_eq!(res.groups.len(), 0);
            assert_eq!(res.mnemonics.len(), 0);
            assert_eq!(res.jumps.len(), 0);
        }

        {
            let maybe_res = main.next_match(&mut src.iter().seek(2),2,());

            assert!(maybe_res.is_some());
            let res = maybe_res.unwrap();

            assert_eq!(res.address, 2);
            assert_eq!(res.tokens.len(), 3);
            assert_eq!(res.tokens[0], 3);
            assert_eq!(res.tokens[1], 2);
            assert_eq!(res.tokens[2], 2);
            assert_eq!(res.groups.len(), 0);
            assert_eq!(res.mnemonics.len(), 0);
            assert_eq!(res.jumps.len(), 0);
        }
    }

    #[test]
    fn decode_macro() {
        let lock_prfx = new_disassembler!(TestArchShort =>
            [ 0x06 ] = &|_| { true }
        );

        new_disassembler!(TestArchShort =>
            [ 22 , 21, lock_prfx ] = &|_| { true },
            [ "....11 d@00"         ] = &|_| true,
            [ "....11 d@00", ".. d@0011. 0" ] = &|_| true
        );
    }

    fn fixture() -> (Rc<Disassembler<TestArchShort>>,Rc<Disassembler<TestArchShort>>,Rc<Disassembler<TestArchShort>>,OpaqueLayer) {
        let sub = new_disassembler!(TestArchShort =>
            [ 2 ] = |st: &mut State<TestArchShort>| {
                let next = st.address;
                st.mnemonic(2,"BA","",vec!(),&|_| {});
                st.jump(Rvalue::Constant(next + 2),Guard::always());
                true
            });
        let sub2 = new_disassembler!(TestArchShort =>
            [ 8 ] = &|_| false);

        let main = new_disassembler!(TestArchShort =>
            [ 1, sub ] = &|_| true,
            [ 1 ] = |st: &mut State<TestArchShort>| {
                let next = st.address;
                st.mnemonic(1,"A","",vec!(),&|_| {});
                st.jump(Rvalue::Constant(next + 1),Guard::always());
                true
            },
            [ "0 k@..... 11" ] = |st: &mut State<TestArchShort>| {
                let next = st.address;
                st.mnemonic(1,"C","",vec!(),&|_| {});
                st.jump(Rvalue::Constant(next + 1),Guard::always());
                true
            },
            _ = |st: &mut State<TestArchShort>| {
                let next = st.address;
                st.mnemonic(1,"UNK","",vec!(),&|_| {});
                st.jump(Rvalue::Constant(next + 1),Guard::always());
                true
            }
		);

        (sub,sub2,main,OpaqueLayer::wrap(vec!(1,1,2,1,3,8,1,8)))
	}

    #[test]
    fn single_decoder() {
        let (_,_,main,def) = fixture();
        let maybe_res = main.next_match(&mut def.iter(),0,());

        assert!(maybe_res.is_some());
        let res = maybe_res.unwrap();

        assert_eq!(res.address, 0);
        assert_eq!(res.tokens.len(), 1);
        assert_eq!(res.tokens[0], 1);
        assert_eq!(res.groups.len(), 0);
        assert_eq!(res.mnemonics.len(), 1);
        assert_eq!(res.mnemonics[0].opcode, "A".to_string());
        assert_eq!(res.mnemonics[0].area, Bound::new(0,1));
        assert_eq!(res.mnemonics[0].instructions.len(), 0);
        assert_eq!(res.jumps.len(), 1);

        if let &(0,Rvalue::Constant(1),ref g) = &res.jumps[0] {
            assert_eq!(g, &Guard::always());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn sub_decoder() {
        let (_,_,main,def) = fixture();
        let maybe_res = main.next_match(&mut def.iter().cut(&(1..def.len())),1,());

        assert!(maybe_res.is_some());
        let res = maybe_res.unwrap();

        assert_eq!(res.address, 1);
        assert_eq!(res.tokens.len(), 2);
        assert_eq!(res.tokens[0], 1);
        assert_eq!(res.tokens[1], 2);
        assert_eq!(res.groups.len(), 0);
        assert_eq!(res.mnemonics.len(), 1);
        assert_eq!(res.mnemonics[0].opcode, "BA".to_string());
        assert_eq!(res.mnemonics[0].area, Bound::new(1,3));
        assert_eq!(res.mnemonics[0].instructions.len(), 0);
        assert_eq!(res.jumps.len(), 1);

        if let &(1,Rvalue::Constant(3),ref g) = &res.jumps[0] {
            assert_eq!(g, &Guard::always());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn semantic_false() {
        let (_,sub2,_,def) = fixture();
        let maybe_res = sub2.next_match(&mut def.iter().cut(&(7..def.len())),7,());

        assert!(maybe_res.is_none());
    }

    #[test]
    fn default_pattern() {
        let (_,_,main,def) = fixture();
        let maybe_res = main.next_match(&mut def.iter().cut(&(7..def.len())),7,());

        assert!(maybe_res.is_some());
        let res = maybe_res.unwrap();

        assert_eq!(res.address, 7);
        assert_eq!(res.tokens.len(), 1);
        assert_eq!(res.tokens[0], 8);
        assert_eq!(res.groups.len(), 0);
        assert_eq!(res.mnemonics.len(), 1);
        assert_eq!(res.mnemonics[0].opcode, "UNK".to_string());
        assert_eq!(res.mnemonics[0].area, Bound::new(7,8));
        assert_eq!(res.mnemonics[0].instructions.len(), 0);
        assert_eq!(res.jumps.len(), 1);

        if let &(7,Rvalue::Constant(8),ref g) = &res.jumps[0] {
            assert_eq!(g, &Guard::always());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn slice() {
        let (_,_,main,def) = fixture();
        let maybe_res = main.next_match(&mut def.iter().cut(&(1..2)),1,());

        assert!(maybe_res.is_some());
        let res = maybe_res.unwrap();

        assert_eq!(res.address, 1);
        assert_eq!(res.tokens.len(), 1);
        assert_eq!(res.tokens[0], 1);
        assert_eq!(res.groups.len(), 0);
        assert_eq!(res.mnemonics.len(), 1);
        assert_eq!(res.mnemonics[0].opcode, "A".to_string());
        assert_eq!(res.mnemonics[0].area, Bound::new(1,2));
        assert_eq!(res.mnemonics[0].instructions.len(), 0);
        assert_eq!(res.jumps.len(), 1);

        if let &(1,Rvalue::Constant(2),ref g) = &res.jumps[0] {
            assert_eq!(g, &Guard::always());
        } else {
            assert!(false);
        }
     }

    #[test]
    fn empty() {
        let (_,_,main,def) = fixture();
        let maybe_res = main.next_match(&mut def.iter().cut(&(0..0)),0,());

        assert!(maybe_res.is_none());
    }

    #[test]
    fn capture_group() {
        let (_,_,main,def) = fixture();
        let maybe_res = main.next_match(&mut def.iter().cut(&(4..def.len())),4,());

        assert!(maybe_res.is_some());
        let res = maybe_res.unwrap();

        assert_eq!(res.address, 4);
        assert_eq!(res.tokens.len(), 1);
        assert_eq!(res.tokens[0], 3);
        assert_eq!(res.groups.len(), 1);
        assert_eq!(res.groups, vec!(("k".to_string(),0)));
        assert_eq!(res.mnemonics.len(), 1);
        assert_eq!(res.mnemonics[0].opcode, "C".to_string());
        assert_eq!(res.mnemonics[0].area, Bound::new(4,5));
        assert_eq!(res.mnemonics[0].instructions.len(), 0);
        assert_eq!(res.jumps.len(), 1);

        if let &(4,Rvalue::Constant(5),ref g) = &res.jumps[0] {
            assert_eq!(g, &Guard::always());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn empty_capture_group() {
        let def = OpaqueLayer::wrap(vec!(127));
        let dec = new_disassembler!(TestArchShort =>
            ["01 a@.. 1 b@ c@..."] = |st: &mut State<TestArchShort>| {
                st.mnemonic(1, "1","",vec!(),&|_| {});
                true
            }
        );
        let maybe_res = dec.next_match(&mut def.iter(),0,());

        assert!(maybe_res.is_some());
        let res = maybe_res.unwrap();

        assert_eq!(res.address, 0);
        assert_eq!(res.tokens.len(), 1);
        assert_eq!(res.tokens[0], 127);
        assert!(res.groups == vec!(("a".to_string(),3),("c".to_string(),7)) || res.groups == vec!(("c".to_string(),7),("a".to_string(),3)));
        assert_eq!(res.mnemonics.len(), 1);
        assert_eq!(res.mnemonics[0].opcode, "1".to_string());
        assert_eq!(res.mnemonics[0].area, Bound::new(0,1));
        assert_eq!(res.mnemonics[0].instructions.len(), 0);
        assert_eq!(res.jumps.len(), 0);
    }

    #[test]
    #[should_panic]
    fn too_long_capture_group() {
        new_disassembler!(TestArchShort => [ "k@........." ] = &|_| { true });
    }

    #[test]
    #[should_panic]
    fn too_long_token_pattern() {
        new_disassembler!(TestArchShort => [ "111111111" ] = &|_| { true });
    }

    #[test]
    #[should_panic]
    fn too_short_token_pattern() {
        new_disassembler!(TestArchShort => [ "1111111" ] = &|_| { true });
    }

    #[test]
    #[should_panic]
    fn invalid_char_in_token_pattern() {
        new_disassembler!(TestArchShort => [ "101/1010" ] = &|_| { true });
    }

    #[test]
    #[should_panic]
    fn invalid_token_pattern() {
        new_disassembler!(TestArchShort => [ "a111111" ] = &|_| { true });
    }

    #[test]
    fn wide_token() {
        let def = OpaqueLayer::wrap(vec!(0x11,0x22,0x33,0x44,0x55,0x44));
        let dec = new_disassembler!(TestArchWide =>
            [0x2211] = |s: &mut State<TestArchWide>|
            {
                let a = s.address;
                s.mnemonic(2,"A","",vec!(),&|_| {});
                s.jump(Rvalue::Constant(a + 2),Guard::always());
                true
            },

            [0x4433] = |s: &mut State<TestArchWide>|
            {
                let a = s.address;
                s.mnemonic(2,"B","",vec!(),&|_| {});
                s.jump(Rvalue::Constant(a + 2),Guard::always());
                s.jump(Rvalue::Constant(a + 4),Guard::always());
                true
            },

            [0x4455] = |s: &mut State<TestArchWide>|
            {
                s.mnemonic(2, "C","",vec!(),&|_| {});
                true
            }
        );

        let maybe_res = dec.next_match(&mut def.iter(),0,());

        assert!(maybe_res.is_some());
        let res = maybe_res.unwrap();

        assert_eq!(res.address, 0);
        assert_eq!(res.tokens.len(), 1);
        assert_eq!(res.tokens[0], 0x2211);
        assert_eq!(res.mnemonics.len(), 1);
        assert_eq!(res.mnemonics[0].opcode, "A".to_string());
        assert_eq!(res.mnemonics[0].area, Bound::new(0,2));
        assert_eq!(res.mnemonics[0].instructions.len(), 0);
        assert_eq!(res.jumps.len(), 1);
    }

    #[test]
    fn optional() {
        let def = OpaqueLayer::wrap(vec!(127,126,125,127,125));
        let dec = new_disassembler!(TestArchShort =>
            [127, opt!(126), 125] = |st: &mut State<TestArchShort>|
            {
                let l = st.tokens.len();
                st.mnemonic(l, "1", "", vec!(),&|_| {});
                true
            }
        );

        dec.to_dot();

        {
            let maybe_res = dec.next_match(&mut def.iter(),0,());

            assert!(maybe_res.is_some());
            let res = maybe_res.unwrap();

            assert_eq!(res.address, 0);
            assert_eq!(res.tokens.len(), 3);
            assert_eq!(res.tokens, vec!(127,126,125));
            assert_eq!(res.mnemonics.len(), 1);
            assert_eq!(res.mnemonics[0].opcode, "1".to_string());
            assert_eq!(res.mnemonics[0].area, Bound::new(0,3));
            assert_eq!(res.mnemonics[0].instructions.len(), 0);
            assert_eq!(res.jumps.len(), 0);
        }

        {
            let maybe_res = dec.next_match(&mut def.iter().cut(&(3..5)),3,());

            assert!(maybe_res.is_some());
            let res = maybe_res.unwrap();

            assert_eq!(res.address, 3);
            assert_eq!(res.tokens.len(), 2);
            assert_eq!(res.tokens, vec!(127,125));
            assert_eq!(res.mnemonics.len(), 1);
            assert_eq!(res.mnemonics[0].opcode, "1".to_string());
            assert_eq!(res.mnemonics[0].area, Bound::new(3,5));
            assert_eq!(res.mnemonics[0].instructions.len(), 0);
            assert_eq!(res.jumps.len(), 0);
        }
    }

    #[test]
    fn optional_group() {
        let def = OpaqueLayer::wrap(vec!(127,126));
        let dec = new_disassembler!(TestArchShort =>
            [opt!("011 a@. 1111"), "0111111 b@.", "011 c@. 1110"] = |st: &mut State<TestArchShort>|
            {
                assert_eq!(st.get_group("b"),1);
                assert_eq!(st.get_group("c"),1);

                let l = st.tokens.len();
                st.mnemonic(l, "1", "", vec!(),&|_| {});
                true
            }
        );

        {
            let maybe_res = dec.next_match(&mut def.iter(),0,());

            assert!(maybe_res.is_some());
            let res = maybe_res.unwrap();

            assert_eq!(res.address, 0);
            assert_eq!(res.tokens.len(), 2);
            assert_eq!(res.tokens, vec!(127,126));
            assert_eq!(res.mnemonics.len(), 1);
            assert_eq!(res.mnemonics[0].opcode, "1".to_string());
            assert_eq!(res.mnemonics[0].area, Bound::new(0,2));
            assert_eq!(res.mnemonics[0].instructions.len(), 0);
            assert_eq!(res.jumps.len(), 0);
        }
    }

    #[test]
    fn fixed_capture_group_contents() {
        let def = OpaqueLayer::wrap(vec!(127,255));
        let dec = new_disassembler!(TestArchShort =>
            [ "01111111", "a@11111111" ] = |st: &mut State<TestArchShort>|
            {
                let l = st.tokens.len();
                st.mnemonic(l, "1", "", vec!(),&|_| {});
                true
            }
        );

        let maybe_res = dec.next_match(&mut def.iter(),0,());

        assert!(maybe_res.is_some());
        let res = maybe_res.unwrap();

        assert_eq!(res.address, 0);
        assert_eq!(res.tokens.len(), 2);
        assert_eq!(res.tokens, vec!(127,255));
        assert_eq!(res.groups, vec!(("a".to_string(),255)));
        assert_eq!(res.mnemonics.len(), 1);
        assert_eq!(res.mnemonics[0].opcode, "1".to_string());
        assert_eq!(res.mnemonics[0].area, Bound::new(0,2));
        assert_eq!(res.mnemonics[0].instructions.len(), 0);
        assert_eq!(res.jumps.len(), 0);
    }
}
