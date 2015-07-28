#![macro_use]

use value::Rvalue;
use mnemonic::Mnemonic;
use guard::Guard;
use std::rc::Rc;
use num::traits::{Zero,One,NumCast,cast};
use std::slice::Iter;
use std::fmt::{Display,Debug};
use std::ops::{BitAnd,BitOr,Shl,Shr,Not};
use std::collections::HashMap;
use std::mem::size_of;
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
    pub jumps: Vec<(Rvalue,Guard)>,

    next_address: u64,
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
            next_address: a,
            configuration: c,
        }
    }

    pub fn get_group(&self,n: &str) -> u64 {
        self.groups.iter().find(|x| x.0 == n.to_string()).unwrap().1.clone()
    }

    pub fn mnemonic<F: FnOnce(&mut CodeGen) -> ()>(&mut self,len: usize, n: &str, fmt: &str, ops: Vec<Rvalue>, f: F) {
        self.mnemonic_dynargs(len,n,fmt,|cg: &mut CodeGen| -> Vec<Rvalue> {
            f(cg);
            ops.clone()
        });
    }

    pub fn mnemonic_dynargs<F>(&mut self,len: usize, n: &str, fmt: &str, f: F)
    where F: FnOnce(&mut CodeGen) -> Vec<Rvalue> {
        let mut cg = CodeGen::new();
        let ops = f(&mut cg);

        self.mnemonics.push(Mnemonic::new(
                self.next_address..(self.next_address + (len as u64)),
                n.to_string(),
                fmt.to_string(),
                ops.iter(),
                cg.instructions.iter()));
        self.next_address += len as u64;
    }

    pub fn jump(&mut self,v: Rvalue,g: Guard) {
        self.jumps.push((v,g));
    }
}

#[derive(Clone)]
pub struct Match<A: Architecture> {
    patterns: Vec<(A::Token,A::Token)>,
    actions: Vec<Rc<Action<A>>>,
    groups: Vec<(String,Vec<A::Token>)>
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

impl<A: Architecture> Expr<A> {
    pub fn matches(&self) -> Vec<Match<A>> {
        match self {
            &Expr::Pattern(ref s) => {
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
                                //return Vec::new();
                            } else {
                                read_pat = true;

                                if cur_group == "" {
                                    panic!("Pattern syntax error: anonymous groups not allowed in '{}'",s);
                                    //return Vec::new();
                                }

                                groups.insert(cur_group.clone(),A::Token::zero());
                            }
                        },
                        ' ' => {
                            read_pat = false;
                            cur_group = "".to_string();
                        },
                        '.' => {
                            if read_pat && cur_group != "" {
                                *groups.get_mut(&cur_group).unwrap() = groups.get(&cur_group).unwrap().clone() | (A::Token::one() << ((bit - 1) as usize));
                            }

                            bit -= 1;
                        },
                        '0' | '1' => {
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
                                //return Vec::new();
                            } else {
                                cur_group.push(c);
                            }
                        },
                        _ => {
                            panic!("Pattern syntax error: invalid character '{}' in '{}'",c,s);
                            //return Vec::new();
                        }
                    }
                }

                if bit != 0 {
                    panic!("Pattern syntax error: invalid pattern length in '{}'",s);
                }

                vec!(Match::<A>{
                    patterns: vec!((pat,mask)),
                    groups: groups.iter().filter_map(|x| {
                        if *x.1 != A::Token::zero() {
                            Some((x.0.clone(),vec!(x.1.clone())))
                        } else {
                            None
                        }
                    }).collect(),
                    actions: vec!()
                })
            },
            &Expr::Terminal(ref i) => vec!(Match::<A>{
                patterns: vec!((i.clone(),!A::Token::zero())),
                groups: vec!(),
                actions: vec!(),
            }),
            &Expr::Subdecoder(ref m) => m.matches.clone(),
            &Expr::Optional(ref e) => {
                let mut ms = e.matches();
                ms.push(Match::<A>{
                    patterns: vec!(),
                    groups: vec!(),
                    actions: vec!()
                });
                ms
            }
        }
    }
}

pub struct Disassembler<A: Architecture> {
    pub matches: Vec<Match<A>>,
    default: Option<Action<A>>,
}

impl<A: Architecture> Disassembler<A> {
    pub fn new() -> Disassembler<A> {
        Disassembler::<A> {
            matches: Vec::new(),
            default: None,
        }
    }

    pub fn set_default(&mut self,f: Action<A>) {
        self.default = Some(f);
    }

    fn combine_expr(mut i: Iter<Expr<A>>, a: Action<A>) -> Vec<Match<A>> {
        match i.next() {
            Some(e) => {
                let rest = Self::combine_expr(i,a);
                let mut ret = Vec::new();


                for _match in (*e).matches() {
                    for pre in &rest {
                        let mut m = _match.clone();

                        for x in &pre.patterns {
                            m.patterns.push(x.clone());
                        }

                        for x in &pre.actions {
                            m.actions.push(x.clone());
                        }

                        for x in &pre.groups {
                            let mut new = true;
                            for y in m.groups.iter_mut() {
                                if y.0 == x.0 {
                                    for p in &x.1 {
                                        y.1.push(p.clone());
                                    }
                                    new = false;
                                }
                            }

                            if new {
                                m.groups.push(x.clone());
                            }
                        }

                        ret.push(Match::<A>{
                            patterns: m.patterns,
                            actions: m.actions,
                            groups: m.groups
                        });
                    }
                }

                ret
            },
            None => vec!(Match::<A>{
                patterns: vec!(),
                actions: vec!(Rc::new(a)),
                groups: vec!(),
            })
        }
    }

    pub fn add_expr(&mut self, e: Vec<Expr<A>>, a: Action<A>) {
        for x in Self::combine_expr(e.iter(),a) {
            self.matches.push(x);
        }
    }

    pub fn next_match(&self,i: &mut LayerIter, _st: State<A>) -> Option<State<A>> {
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

        for opt in &self.matches {
            let pattern = &opt.patterns;
            let actions = &opt.actions;

            if !min_len(pattern.len(),&mut tokens,&mut j) {
                continue;
            }

            let is_match = pattern.iter().zip(tokens.iter()).all(|p| {
                let pat = (p.0).0.clone();
                let msk = (p.0).1.clone();
                let tok = p.1.clone();

                (msk & tok) == pat
            });

            if is_match {
                let mut grps = HashMap::<String,u64>::new();
                let mut st = _st.clone();

                for cap in &opt.groups {
                    let masks = &cap.1;
                    let mut res: u64 = grps.get(&cap.0).unwrap_or(&0).clone();

                    for tok_msk in tokens.iter().zip(masks.iter()) {
                        if *tok_msk.1 != A::Token::zero() {
                            for rbit in (0..(size_of::<A::Token>() * 8)) {
                                let bit = (size_of::<A::Token>() * 8) - rbit - 1;
                                let mask = if bit > 0 {
                                    A::Token::one() << bit
                                } else {
                                    A::Token::one()
                                };

                                let a = mask.clone() & tok_msk.1.clone();

                                if a != A::Token::zero() {
                                    res <<= 1;

                                    if tok_msk.0.clone() & a != A::Token::zero() {
                                        res |= 1;
                                    }
                                }
                            }
                        }
                    }

                    grps.insert(cap.0.clone(),res);
                }

                st.tokens = tokens.iter().take(pattern.len()).cloned().collect();
                st.groups = grps.iter().map(|x| (x.0.clone(),x.1.clone())).collect::<Vec<(String,u64)>>();

                if actions.iter().all(|x| x(&mut st)) {
                    return Some(st);
                }
            }
        }

        if self.default.is_some() && min_len(1,&mut tokens,&mut j) {
            let mut st = _st.clone();

            st.tokens = vec!(tokens.iter().next().unwrap().clone());

            if self.default.unwrap()(&mut st) {
                return Some(st);
            }
        }

        None
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
                dis.add_expr(__x,fuc);
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
                dis.add_expr(__x,fuc);
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

    #[test]
    fn combine_expr() {
        let sub = new_disassembler!(u8 =>
            [ 1 ] = |_| { true },
            [ 2, 2 ] = |_| { true }
        );

        let main = new_disassembler!(u8 =>
            [ 3, sub ] = |_| { true }
        );

        for x in &main.matches {
            assert!(x.patterns == vec!((3,255),(1,255)) || x.patterns == vec!((3,255),(2,255),(2,255)));
        }
    }

    #[test]
    fn decode_macro() {
        let lock_prfx = new_disassembler!(u8 =>
            [ 0x06 ] = |_| { true }
        );

        new_disassembler!(u8 =>
            [ 22 , 21, lock_prfx ] = |_| { true },
            [ "....11 d@00"         ] = |_| true,
            [ "....11 d@00", ".. d@0011. 0" ] = |_| true
        );
    }

    fn fixture() -> (Rc<Disassembler<u8>>,Rc<Disassembler<u8>>,Rc<Disassembler<u8>>,OpaqueLayer) {
        let sub = new_disassembler!(u8 =>
            [ 2 ] = |st: &mut State<u8>| {
                let next = st.address;
                st.mnemonic(2,"BA","",vec!(),|_| {});
                st.jump(Rvalue::Constant(next + 2),Guard::always());
                true
            });
        let sub2 = new_disassembler!(u8 =>
            [ 8 ] = |_| false);

        let main = new_disassembler!(u8 =>
            [ 1, sub ] = |_| true,
            [ 1 ] = |st: &mut State<u8>| {
                let next = st.address;
                st.mnemonic(1,"A","",vec!(),|_| {});
                st.jump(Rvalue::Constant(next + 1),Guard::always());
                true
            },
            [ "0 k@..... 11" ] = |st: &mut State<u8>| {
                let next = st.address;
                st.mnemonic(1,"C","",vec!(),|_| {});
                st.jump(Rvalue::Constant(next + 1),Guard::always());
                true
            },
            _ = |st: &mut State<u8>| {
                let next = st.address;
                st.mnemonic(1,"UNK","",vec!(),|_| {});
                st.jump(Rvalue::Constant(next + 1),Guard::always());
                true
            }
		);

        (sub,sub2,main,OpaqueLayer::wrap(vec!(1,1,2,1,3,8,1,8)))
	}

    #[test]
    fn single_decoder() {
        let (_,_,main,def) = fixture();
        let st = State::<u8>::new(0);
        let maybe_res = main.next_match(&mut def.iter(),st);

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

        if let &(Rvalue::Constant(1),ref g) = &res.jumps[0] {
            assert_eq!(g, &Guard::always());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn sub_decoder() {
        let (_,_,main,def) = fixture();
        let st = State::<u8>::new(1);
        let maybe_res = main.next_match(&mut def.iter().cut(&(1..def.len())),st);

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

        if let &(Rvalue::Constant(3),ref g) = &res.jumps[0] {
            assert_eq!(g, &Guard::always());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn semantic_false() {
        let (_,sub2,_,def) = fixture();
        let st = State::<u8>::new(7);
        let maybe_res = sub2.next_match(&mut def.iter().cut(&(7..def.len())),st);

        assert!(maybe_res.is_none());
    }

    #[test]
    fn default_pattern() {
        let (_,_,main,def) = fixture();
        let st = State::<u8>::new(7);
        let maybe_res = main.next_match(&mut def.iter().cut(&(7..def.len())),st);

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

        if let &(Rvalue::Constant(8),ref g) = &res.jumps[0] {
            assert_eq!(g, &Guard::always());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn slice() {
        let (_,_,main,def) = fixture();
        let st = State::<u8>::new(1);
        let maybe_res = main.next_match(&mut def.iter().cut(&(1..2)),st);

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

        if let &(Rvalue::Constant(2),ref g) = &res.jumps[0] {
            assert_eq!(g, &Guard::always());
        } else {
            assert!(false);
        }
     }

    #[test]
    fn empty() {
        let (_,_,main,def) = fixture();
        let st = State::<u8>::new(0);
        let maybe_res = main.next_match(&mut def.iter().cut(&(0..0)),st);

        assert!(maybe_res.is_none());
    }

    #[test]
    fn capture_group() {
        let (_,_,main,def) = fixture();
        let st = State::<u8>::new(4);
        let maybe_res = main.next_match(&mut def.iter().cut(&(4..def.len())),st);

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

        if let &(Rvalue::Constant(5),ref g) = &res.jumps[0] {
            assert_eq!(g, &Guard::always());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn empty_capture_group() {
        let st = State::<u8>::new(0);
        let def = OpaqueLayer::wrap(vec!(127));
        let dec = new_disassembler!(u8 =>
            ["01 a@.. 1 b@ c@..."] = |st: &mut State<u8>| {
                st.mnemonic(1, "1","",vec!(),|_| {});
                true
            }
        );
        let maybe_res = dec.next_match(&mut def.iter(),st);

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
        new_disassembler!(u8 => [ "k@........." ] = |_| { true });
    }

    #[test]
    #[should_panic]
    fn too_long_token_pattern() {
        new_disassembler!(u8 => [ "111111111" ] = |_| { true });
    }

    #[test]
    #[should_panic]
    fn too_short_token_pattern() {
        new_disassembler!(u8 => [ "1111111" ] = |_| { true });
    }

    #[test]
    #[should_panic]
    fn invalid_char_in_token_pattern() {
        new_disassembler!(u8 => [ "101/1010" ] = |_| { true });
    }

    #[test]
    #[should_panic]
    fn invalid_token_pattern() {
        new_disassembler!(u8 => [ "a111111" ] = |_| { true });
    }

    #[test]
    fn wide_token() {
        let st = State::<u16>::new(0);
        let def = OpaqueLayer::wrap(vec!(0x11,0x22,0x33,0x44,0x55,0x44));
        let dec = new_disassembler!(u16 =>
            [0x1122] = |s: &mut State<u16>|
            {
                let a = s.address;
                s.mnemonic(2,"A","",vec!(),|_| {});
                s.jump(Rvalue::Constant(a + 2),Guard::always());
                true
            },

            [0x3344] = |s: &mut State<u16>|
            {
                let a = s.address;
                s.mnemonic(2,"B","",vec!(),|_| {});
                s.jump(Rvalue::Constant(a + 2),Guard::always());
                s.jump(Rvalue::Constant(a + 4),Guard::always());
                true
            },

            [0x5544] = |s: &mut State<u16>|
            {
                s.mnemonic(2, "C","",vec!(),|_| {});
                true
            }
        );

        let maybe_res = dec.next_match(&mut def.iter(),st);

        assert!(maybe_res.is_some());
        let res = maybe_res.unwrap();

        assert_eq!(res.address, 0);
        assert_eq!(res.tokens.len(), 1);
        assert_eq!(res.tokens[0], 0x1122);
        assert_eq!(res.mnemonics.len(), 1);
        assert_eq!(res.mnemonics[0].opcode, "A".to_string());
        assert_eq!(res.mnemonics[0].area, Bound::new(0,2));
        assert_eq!(res.mnemonics[0].instructions.len(), 0);
        assert_eq!(res.jumps.len(), 1);
    }

    #[test]
    fn optional() {
        let def = OpaqueLayer::wrap(vec!(127,126,125,127,125));
        let dec = new_disassembler!(u8 =>
            [127, opt!(126), 125] = |st: &mut State<u8>|
            {
                let l = st.tokens.len();
                st.mnemonic(l, "1", "", vec!(),|_| {});
                true
            }
        );

        for x in &dec.matches {
            println!("{:?}",x.patterns);
        }

        {
            let st = State::<u8>::new(0);
            let maybe_res = dec.next_match(&mut def.iter(),st);

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
            let st = State::<u8>::new(3);
            let maybe_res = dec.next_match(&mut def.iter().cut(&(3..5)),st);

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
    fn fixed_capture_group_contents() {
        let def = OpaqueLayer::wrap(vec!(127,255));
        let dec = new_disassembler!(u8 =>
            [ "01111111", "a@11111111" ] = |st: &mut State<u8>|
            {
                let l = st.tokens.len();
                st.mnemonic(l, "1", "", vec!(),|_| {});
                true
            }
        );

        for d in &dec.matches {
            println!("{:?}",d.groups);
        }
        let st = State::<u8>::new(0);
        let maybe_res = dec.next_match(&mut def.iter(),st);

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
