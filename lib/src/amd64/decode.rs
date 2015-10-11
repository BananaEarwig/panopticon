use value::{Lvalue,Rvalue,Endianess};
use disassembler::State;
use amd64::*;
use codegen::CodeGen;
use guard::Guard;

fn byte(o: Rvalue) -> Lvalue {
    Lvalue::Memory{
        offset: Box::new(o),
        bytes: 1,
        endianess: Endianess::Little,
        name: "ram".to_string()
    }
}

fn word(o: Rvalue) -> Lvalue {
    Lvalue::Memory{
        offset: Box::new(o),
        bytes: 2,
        endianess: Endianess::Little,
        name: "ram".to_string()
    }
}

fn dword(o: Rvalue) -> Lvalue {
    Lvalue::Memory{
        offset: Box::new(o),
        bytes: 4,
        endianess: Endianess::Little,
        name: "ram".to_string()
    }
}

fn qword(o: Rvalue) -> Lvalue {
    Lvalue::Memory{
        offset: Box::new(o),
        bytes: 8,
        endianess: Endianess::Little,
        name: "ram".to_string()
    }
}

fn oword(o: Rvalue) -> Lvalue {
    Lvalue::Memory{
        offset: Box::new(o),
        bytes: 16,
        endianess: Endianess::Little,
        name: "ram".to_string()
    }
}

pub fn decode_m(sm: &mut State<Amd64>) -> Rvalue {
    sm.configuration.rm.as_ref().unwrap().to_rv()
}

pub fn decode_d(sm: &mut State<Amd64>) -> Rvalue {
    if let Some(Rvalue::Constant(c)) = sm.configuration.imm {
        if c <= 0xffffffff {
            Rvalue::Constant(c >> 16 | ((c & 0xffff) << 16))
        } else {
            Rvalue::Constant(c >> 32 | ((c & 0xffffffff) << 32))
        }
    } else {
        unreachable!()
    }
}

pub fn decode_imm(sm: &mut State<Amd64>) -> Rvalue {
    sm.configuration.imm.clone().unwrap()
}

pub fn decode_moffs(sm: &mut State<Amd64>) -> Rvalue {
    sm.configuration.moffs.clone().unwrap()
}

pub fn decode_rm1(sm: &mut State<Amd64>) -> Rvalue {
    sm.configuration.rm.as_ref().unwrap().to_rv()
}

pub fn decode_i(sm: &mut State<Amd64>) -> (Rvalue,Rvalue) {
    match &sm.configuration.operand_size {
        &OperandSize::Eight => (AH.to_rv(),sm.configuration.imm.clone().unwrap()),
        &OperandSize::Sixteen => (AX.to_rv(),sm.configuration.imm.clone().unwrap()),
        &OperandSize::ThirtyTwo => (EAX.to_rv(),sm.configuration.imm.clone().unwrap()),
        &OperandSize::SixtyFour => (RAX.to_rv(),sm.configuration.imm.clone().unwrap()),
        &OperandSize::HundredTwentyEight => panic!("No 128 bit register in x86!")
    }
}

pub fn decode_rm(sm: &mut State<Amd64>) -> (Rvalue,Rvalue) {
    if let (&Some(ref reg),&Some(ref rm)) = (&sm.configuration.reg,&sm.configuration.rm) {
        (reg.to_rv(),rm.to_rv())
    } else {
        unreachable!();
    }
}

pub fn decode_mr(sm: &mut State<Amd64>) -> (Rvalue,Rvalue) {
    let (a,b) = decode_rm(sm);
    (b,a)
}

pub fn decode_fd(sm: &mut State<Amd64>) -> (Rvalue,Rvalue) {
    (select_reg(&sm.configuration.operand_size,0,sm.configuration.rex).to_rv(),
    select_mem(&sm.configuration.operand_size,sm.configuration.moffs.clone().unwrap()).to_rv())
}

pub fn decode_td(sm: &mut State<Amd64>) -> (Rvalue,Rvalue) {
    let (a,b) = decode_fd(sm);
    (b,a)
}

pub fn decode_msreg(sm: &mut State<Amd64>) -> (Rvalue,Rvalue) {
    let (a,b) = decode_sregm(sm);
    (b,a)
}

pub fn decode_sregm(sm: &mut State<Amd64>) -> (Rvalue,Rvalue) {
    if let (&Some(ref reg),&Some(ref rm)) = (&sm.configuration.reg,&sm.configuration.rm) {
        if *reg == *AX || *reg == *EAX  {
            (ES.to_rv(),rm.to_rv())
        } else if *reg == *CX || *reg == *ECX  {
            (CS.to_rv(),rm.to_rv())
        } else if *reg == *DX || *reg == *EDX  {
            (SS.to_rv(),rm.to_rv())
        } else if *reg == *BX || *reg == *EBX  {
            (DS.to_rv(),rm.to_rv())
        } else if *reg == *SP || *reg == *ESP  {
            (FS.to_rv(),rm.to_rv())
        } else if *reg == *BP || *reg == *EBP  {
            (GS.to_rv(),rm.to_rv())
        } else {
            unreachable!()
        }
    } else {
        unreachable!()
    }
}

pub fn decode_dbgrm(sm: &mut State<Amd64>) -> (Rvalue,Rvalue) {
    if let (&Some(ref reg),&Some(ref rm)) = (&sm.configuration.reg,&sm.configuration.rm) {
        if *reg == *RAX || *reg == *EAX  {
            (DR0.to_rv(),rm.to_rv())
        } else if *reg == *RCX || *reg == *ECX  {
            (DR1.to_rv(),rm.to_rv())
        } else if *reg == *RDX || *reg == *EDX  {
            (DR2.to_rv(),rm.to_rv())
        } else if *reg == *RBX || *reg == *EBX  {
            (DR3.to_rv(),rm.to_rv())
        } else if *reg == *RSP || *reg == *ESP  {
            (DR4.to_rv(),rm.to_rv())
        } else if *reg == *RBP || *reg == *EBP  {
            (DR5.to_rv(),rm.to_rv())
        } else if *reg == *RDI || *reg == *EDI  {
            (DR7.to_rv(),rm.to_rv())
        } else if *reg == *RSI || *reg == *ESI  {
            (DR6.to_rv(),rm.to_rv())
        } else {
            unreachable!()
        }
    } else {
        unreachable!()
    }
}

pub fn decode_rmdbg(sm: &mut State<Amd64>) -> (Rvalue,Rvalue) {
    let (a,b) = decode_dbgrm(sm);
    (b,a)
}

pub fn decode_ctrlrm(sm: &mut State<Amd64>) -> (Rvalue,Rvalue) {
    if let (&Some(ref reg),&Some(ref rm)) = (&sm.configuration.reg,&sm.configuration.rm) {
        if *reg == *RAX || *reg == *EAX  {
            (CR0.to_rv(),rm.to_rv())
        } else if *reg == *RDX || *reg == *EDX  {
            (CR2.to_rv(),rm.to_rv())
        } else if *reg == *RBX || *reg == *EBX  {
            (CR3.to_rv(),rm.to_rv())
        } else if *reg == *RSP || *reg == *ESP  {
            (CR4.to_rv(),rm.to_rv())
        } else if *reg == *R8 || *reg == *R9W  {
            (CR8.to_rv(),rm.to_rv())
        } else {
            unreachable!()
        }
    } else {
        unreachable!()
    }
}

pub fn decode_rmctrl(sm: &mut State<Amd64>) -> (Rvalue,Rvalue) {
    let (a,b) = decode_ctrlrm(sm);
    (b,a)
}

pub fn decode_mi(sm: &mut State<Amd64>) -> (Rvalue,Rvalue) {
    if let (&Some(ref rm),&Some(ref imm)) = (&sm.configuration.rm,&sm.configuration.imm) {
        (rm.to_rv(),imm.clone())
    } else {
        unreachable!()
    }
}

pub fn decode_m1(_: &mut State<Amd64>) -> (Rvalue,Rvalue) {
    unimplemented!();
}

pub fn decode_mc(_: &mut State<Amd64>) -> (Rvalue,Rvalue) {
    unimplemented!();
}

pub fn decode_ii(sm: &mut State<Amd64>) -> (Rvalue,Rvalue) {
    if let &Some(Rvalue::Constant(c)) = &sm.configuration.imm {
        (Rvalue::Constant(c >> 8),Rvalue::Constant(c & 0xff))
    } else {
        unreachable!()
    }
}

pub fn decode_rvm(_: &mut State<Amd64>) -> (Rvalue,Rvalue,Rvalue) {
    unimplemented!();
}

pub fn decode_rmv(_: &mut State<Amd64>) -> (Rvalue,Rvalue,Rvalue) {
    unimplemented!();
}

pub fn decode_rmi(sm: &mut State<Amd64>) -> (Rvalue,Rvalue,Rvalue) {
    if let (&Some(ref reg),&Some(ref rm),&Some(ref imm)) = (&sm.configuration.reg,&sm.configuration.rm,&sm.configuration.imm) {
        (reg.to_rv(),rm.to_rv(),imm.clone())
    } else {
        unreachable!()
    }
}

pub fn decode_mri(sm: &mut State<Amd64>) -> (Rvalue,Rvalue,Rvalue) {
    let (a,b,c) = decode_rmi(sm);
    (b,a,c)
}

pub fn decode_rvmi(_: &mut State<Amd64>) -> (Rvalue,Rvalue,Rvalue,Rvalue) {
    unimplemented!();
}

pub fn decode_reg8(r_reg: u64,rex: bool) -> Lvalue {
    match r_reg {
        0 => AL.clone(),
        1 => CL.clone(),
        2 => DL.clone(),
        3 => BL.clone(),
        4 => if rex { SPL.clone() } else { AH.clone() },
        5 => if rex { BPL.clone() } else { CH.clone() },
        6 => if rex { SIL.clone() } else { DH.clone() },
        7 => if rex { DIL.clone() } else { BH.clone() },
        8 => R8L.clone(),
        9 => R9L.clone(),
        10 => R10L.clone(),
        11 => R11L.clone(),
        12 => R12L.clone(),
        13 => R13L.clone(),
        14 => R14L.clone(),
        15 => R15L.clone(),
        _ => unreachable!()
    }
}

pub fn decode_reg16(r_reg: u64) -> Lvalue {
    match r_reg {
        0 => AX.clone(),
        1 => CX.clone(),
        2 => DX.clone(),
        3 => BX.clone(),
        4 => SP.clone(),
        5 => BP.clone(),
        6 => SI.clone(),
        7 => DI.clone(),
        8 => R8W.clone(),
        9 => R9W.clone(),
        10 => R10W.clone(),
        11 => R11W.clone(),
        12 => R12W.clone(),
        13 => R13W.clone(),
        14 => R14W.clone(),
        15 => R15W.clone(),
        _ => unreachable!()
    }
}

pub fn decode_reg32(r_reg: u64) -> Lvalue {
    match r_reg {
        0 => EAX.clone(),
        1 => ECX.clone(),
        2 => EDX.clone(),
        3 => EBX.clone(),
        4 => ESP.clone(),
        5 => EBP.clone(),
        6 => ESI.clone(),
        7 => EDI.clone(),
        8 => R8D.clone(),
        9 => R9D.clone(),
        10 => R10D.clone(),
        11 => R11D.clone(),
        12 => R12D.clone(),
        13 => R13D.clone(),
        14 => R14D.clone(),
        15 => R15D.clone(),
        _ => unreachable!()
    }
}

pub fn decode_reg64(r_reg: u64) -> Lvalue {
    match r_reg {
        0 => RAX.clone(),
        1 => RCX.clone(),
        2 => RDX.clone(),
        3 => RBX.clone(),
        4 => RSP.clone(),
        5 => RBP.clone(),
        6 => RSI.clone(),
        7 => RDI.clone(),
        8 => R8.clone(),
        9 => R9.clone(),
        10 => R10.clone(),
        11 => R11.clone(),
        12 => R12.clone(),
        13 => R13.clone(),
        14 => R14.clone(),
        15 => R15.clone(),
        _ => unreachable!()
    }
}

pub fn select_reg(os: &OperandSize,r: u64, rex: bool) -> Lvalue {
    match os {
        &OperandSize::Eight => decode_reg8(r,rex),
        &OperandSize::Sixteen => decode_reg16(r),
        &OperandSize::ThirtyTwo => decode_reg32(r),
        &OperandSize::SixtyFour => decode_reg64(r),
        &OperandSize::HundredTwentyEight => panic!("No 128 bit registers in x86!")
    }
}

pub fn select_mem(os: &OperandSize,o: Rvalue) -> Lvalue {
    match os {
        &OperandSize::Eight => byte(o),
        &OperandSize::Sixteen => word(o),
        &OperandSize::ThirtyTwo => dword(o),
        &OperandSize::SixtyFour => qword(o),
        &OperandSize::HundredTwentyEight => oword(o),
    }
}

pub fn decode_modrm(
        _mod: u64,
        b_rm: u64,    // B.R/M
        disp: Option<Rvalue>,
        sib: Option<(u64,u64,u64)>, // scale, X.index, B.base
        os: OperandSize,
        addrsz: AddressSize,
        mode: Mode,
        rex: bool,
        c: &mut CodeGen<Amd64>) -> Lvalue
{
    assert!(_mod < 0x4);
    assert!(b_rm < 0x10);

    match addrsz {
        AddressSize::Sixteen => {
            match _mod {
                0 | 1 | 2 => {
                    let tmp = new_temp(16);

                    if b_rm == 6 {
                        if _mod == 0 {
                            select_mem(&os,disp.unwrap())
                        } else {
                            c.add_i(&tmp,&select_mem(&os,BP.clone().to_rv()),&disp.unwrap());
                            tmp
                        }
                    } else {
                        let base = select_mem(&os,match b_rm {
                            0 => { c.add_i(&tmp,&*BX,&*SI); tmp.clone() },
                            1 => { c.add_i(&tmp,&*BX,&*DI); tmp.clone() },
                            2 => { c.add_i(&tmp,&*BP,&*SI); tmp.clone() },
                            3 => { c.add_i(&tmp,&*BP,&*DI); tmp.clone() },
                            4 => SI.clone(),
                            5 => DI.clone(),
                            7 => BX.clone(),
                            _ => unreachable!(),
                        }.to_rv());

                        if _mod == 0 {
                            base
                        } else {
                            c.add_i(&tmp,&base,&disp.unwrap());
                            tmp
                        }
                    }
                },
                3 => select_reg(&os,b_rm,rex),
                _ => unreachable!()
            }
        },
        AddressSize::ThirtyTwo | AddressSize::SixtyFour => {
            let base = match b_rm {
                0 | 1 | 2 | 3 |
                6 | 7 | 8 | 9 | 10 | 11 |
                14 | 15 => select_reg(&if _mod != 3 && addrsz == AddressSize::SixtyFour { OperandSize::SixtyFour } else { os.clone() },b_rm,rex),

                4 | 12 => if _mod == 3 {
                        select_reg(&os,b_rm,rex)
                    } else {
                        if let Some((scale,index,base)) = sib {
                            decode_sib(_mod,scale,index,base,disp.clone(),os.clone(),c)
                        } else {
                            unreachable!()
                        }
                    },

                5 | 13 => if _mod == 0 {
                    if mode == Mode::Long {
                        if addrsz == AddressSize::SixtyFour {
                            let tmp = new_temp(64);

                            c.add_i(&tmp,disp.as_ref().unwrap(),&*RIP);
                            c.mod_i(&tmp,&tmp,&Rvalue::Constant(0xffffffffffffffff));
                            select_mem(&os,tmp.to_rv())
                        } else {
                            let tmp = new_temp(32);

                            c.add_i(&tmp,disp.as_ref().unwrap(),&*EIP);
                            c.mod_i(&tmp,&tmp,&Rvalue::Constant(0xffffffff));
                            select_mem(&os,tmp.to_rv())
                        }
                    } else {
                        select_mem(&os,disp.clone().unwrap())
                    }
                } else {
                    select_reg(&if _mod != 3 && addrsz == AddressSize::SixtyFour { OperandSize::SixtyFour } else { os.clone() },b_rm,rex)
                },
                _ => unreachable!()
            };

            match _mod {
                0 => select_mem(&os,base.to_rv()),
                1 | 2 => {
                    let tmp = new_temp(os.num_bits());

                    c.add_i(&tmp,&base,disp.as_ref().unwrap());
                    select_mem(&os,tmp.to_rv())
                },
                3 => base,
                _ => unreachable!()
            }
        }
    }
}

fn decode_sib(
    _mod: u64,
    scale: u64,
    x_index: u64,
    b_base: u64,
    disp: Option<Rvalue>,
    os: OperandSize,
    c: &mut CodeGen<Amd64>) -> Lvalue
{
    assert!(_mod <= 3 && scale <= 3 && x_index <= 15 && b_base <= 15);

    match _mod {
        0 => match b_base {
            0 | 1 | 2 | 3 | 4 |
            6 | 7 | 8 | 9 | 10 | 11 | 12 |
            14 | 15 => match x_index {
                0 | 1 | 2 | 3 | 5...15 => {
                    let base = decode_reg64(b_base);
                    let index = decode_reg64(x_index);
                    let tmp = new_temp(64);

                    if scale > 0 {
                        c.mul_i(&tmp,&index,&Rvalue::Constant((1 << (scale & 3)) / 2));
                        c.add_i(&tmp,&base,&tmp);

                        select_mem(&os,tmp.to_rv())
                    } else {
                        c.add_i(&tmp,&base,&index);

                        select_mem(&os,tmp.to_rv())
                    }
                },
                4 => select_mem(&os,Rvalue::Constant((b_base & 7) as u64)),
                _ => unreachable!()
            },
            5 | 13 => match x_index {
                0...3 | 5...15 => {
                    let index = decode_reg64(x_index);
                    let tmp = new_temp(64);

                    if scale > 0 {
                        c.mul_i(&tmp,&index,&Rvalue::Constant((1 << (scale & 3)) / 2));
                        c.add_i(&tmp,&disp.unwrap(),&tmp);

                        select_mem(&os,tmp.to_rv())
                    } else {
                        c.add_i(&tmp,&disp.unwrap(),&index);

                        select_mem(&os,tmp.to_rv())
                    }
                },
                4 => select_mem(&os,disp.unwrap()),
                _ => unreachable!()
            },
            _ => unreachable!()
        },
        1 | 2 => match x_index {
            0...3 | 5...15 => {
                let base = decode_reg64(b_base);
                let index = decode_reg64(x_index);
                let tmp = new_temp(64);

                if scale > 0 {
                    c.mul_i(&tmp,&index,&Rvalue::Constant((1 << (scale & 3)) / 2));
                    c.add_i(&tmp,&tmp,&disp.unwrap());
                    c.add_i(&tmp,&base,&tmp);

                    select_mem(&os,tmp.to_rv())
                } else {
                    c.add_i(&tmp,&index,&disp.unwrap());
                    c.add_i(&tmp,&base,&tmp);

                    select_mem(&os,tmp.to_rv())
                }
            },
            4 => {
                let tmp = new_temp(64);

                c.add_i(&tmp,&decode_reg64(b_base),&disp.unwrap());
                select_mem(&os,tmp.to_rv())
            },
            _ => unreachable!()
        },
        _ => unreachable!()
    }
}

pub fn nonary(opcode: &'static str, sem: fn(&mut CodeGen<Amd64>)) -> Box<Fn(&mut State<Amd64>) -> bool> {
    Box::new(move |st: &mut State<Amd64>| -> bool {
        let len = st.tokens.len();
        let next = st.address + len as u64;

        st.mnemonic_dynargs(len,&opcode,"",&|c| {
            sem(c);
            vec![]
        });
        st.jump(Rvalue::Constant(next),Guard::always());
        true
    })
}

pub fn unary(opcode: &'static str,
              decode: fn(&mut State<Amd64>) -> Rvalue,
              sem: fn(&mut CodeGen<Amd64>,Rvalue)
             ) -> Box<Fn(&mut State<Amd64>) -> bool> {
    Box::new(move |st: &mut State<Amd64>| -> bool {
        let len = st.tokens.len();
        let next = st.address + len as u64;
        let arg = decode(st);

        st.mnemonic_dynargs(len,&opcode,"{64}",&|c| {
            sem(c,arg.clone());
            vec![arg.clone()]
        });
        st.jump(Rvalue::Constant(next),Guard::always());
        true
    })
}

pub fn unary_box(opcode: &'static str,
              decode: fn(&mut State<Amd64>) -> Rvalue,
              sem: Box<Fn(&mut CodeGen<Amd64>,Rvalue)>
             ) -> Box<Fn(&mut State<Amd64>) -> bool> {
    Box::new(move |st: &mut State<Amd64>| -> bool {
        let len = st.tokens.len();
        let next = st.address + len as u64;
        let arg = decode(st);

        st.mnemonic_dynargs(len,&opcode,"{64}",&|c| {
            sem(c,arg.clone());
            vec![arg.clone()]
        });
        st.jump(Rvalue::Constant(next),Guard::always());
        true
    })
}

pub fn unary_c(opcode: &'static str,
                  arg: Rvalue,
                  sem: fn(&mut CodeGen<Amd64>, Rvalue)
                 ) -> Box<Fn(&mut State<Amd64>) -> bool> {
    Box::new(move |st: &mut State<Amd64>| -> bool {
        let len = st.tokens.len();
        let next = st.address + len as u64;

        st.mnemonic_dynargs(len,&opcode,"{64}",&|c| {
            sem(c,arg.clone());
            vec![arg.clone()]
        });
        st.jump(Rvalue::Constant(next),Guard::always());
        true

    })
}

pub fn binary(opcode: &'static str,
              decode: fn(&mut State<Amd64>) -> (Rvalue,Rvalue),
              sem: fn(&mut CodeGen<Amd64>, Rvalue, Rvalue)
             ) -> Box<Fn(&mut State<Amd64>) -> bool> {
    Box::new(move |st: &mut State<Amd64>| -> bool {
        let len = st.tokens.len();
        let next = st.address + len as u64;
        let (arg0,arg1) = decode(st);

        st.mnemonic_dynargs(len,&opcode,"{64}, {64}",&|c| {
            sem(c,arg0.clone(),arg1.clone());
            vec![arg0.clone(),arg1.clone()]
        });
        st.jump(Rvalue::Constant(next),Guard::always());
        true
    })
}

pub fn binary_box(opcode: &'static str,
              decode: fn(&mut State<Amd64>) -> (Rvalue,Rvalue),
              sem: Box<Fn(&mut CodeGen<Amd64>, Rvalue, Rvalue)>
             ) -> Box<Fn(&mut State<Amd64>) -> bool> {
    Box::new(move |st: &mut State<Amd64>| -> bool {
        let len = st.tokens.len();
        let next = st.address + len as u64;
        let (arg0,arg1) = decode(st);

        st.mnemonic_dynargs(len,&opcode,"{64}, {64}",&|c| {
            sem(c,arg0.clone(),arg1.clone());
            vec![arg0.clone(),arg1.clone()]
        });
        st.jump(Rvalue::Constant(next),Guard::always());
        true
    })
}

pub fn binary_rv(opcode: &'static str,
                  _arg0: &Lvalue,
                  decode: fn(&mut State<Amd64>) -> Rvalue,
                  sem: fn(&mut CodeGen<Amd64>, Rvalue, Rvalue)
                 ) -> Box<Fn(&mut State<Amd64>) -> bool> {
    let arg0 = _arg0.clone();
    Box::new(move |st: &mut State<Amd64>| -> bool {
        let len = st.tokens.len();
        let next = st.address + len as u64;
        let arg1 = decode(st);

        st.mnemonic_dynargs(len,&opcode,"{64}, {64}",&|c| {
            sem(c,arg0.to_rv(),arg1.clone());
            vec![arg0.to_rv(),arg1.clone()]
        });
        st.jump(Rvalue::Constant(next),Guard::always());
        true
    })
}

pub fn binary_vr(opcode: &'static str,
                  decode: fn(&mut State<Amd64>) -> Rvalue,
                  _arg1: &Lvalue,
                  sem: fn(&mut CodeGen<Amd64>, Rvalue, Rvalue)
                 ) -> Box<Fn(&mut State<Amd64>) -> bool> {
    let arg1 = _arg1.clone();
    Box::new(move |st: &mut State<Amd64>| -> bool {
        let len = st.tokens.len();
        let next = st.address + len as u64;
        let arg0 = decode(st);

        st.mnemonic_dynargs(len,&opcode,"{64}, {64}",&|c| {
            sem(c,arg0.clone(),arg1.to_rv());
            vec![arg0.clone(),arg1.to_rv()]
        });
        st.jump(Rvalue::Constant(next),Guard::always());
        true
    })
}

pub fn binary_vc(opcode: &'static str,
                  decode: fn(&mut State<Amd64>) -> Rvalue,
                  arg1: Rvalue,
                  sem: fn(&mut CodeGen<Amd64>, Rvalue, Rvalue)
                 ) -> Box<Fn(&mut State<Amd64>) -> bool> {
    Box::new(move |st: &mut State<Amd64>| -> bool {
        let len = st.tokens.len();
        let next = st.address + len as u64;
        let arg0 = decode(st);

        st.mnemonic_dynargs(len,&opcode,"{64}, {64}",&|c| {
            sem(c,arg0.clone(),arg1.clone());
            vec![arg0.clone(),arg1.clone()]
        });
        st.jump(Rvalue::Constant(next),Guard::always());
        true
    })
}

pub fn binary_rr(opcode: &'static str,
                  _arg0: &Lvalue,
                  _arg1: &Lvalue,
                  sem: fn(&mut CodeGen<Amd64>, Rvalue, Rvalue)
                 ) -> Box<Fn(&mut State<Amd64>) -> bool> {
    let arg0 = _arg0.clone();
    let arg1 = _arg1.clone();
    Box::new(move |st: &mut State<Amd64>| -> bool {
        let len = st.tokens.len();
        let next = st.address + len as u64;

        st.mnemonic_dynargs(len,&opcode,"{64}, {64}",&|c| {
            sem(c,arg0.to_rv(),arg1.to_rv());
            vec![arg0.to_rv(),arg1.to_rv()]
        });
        st.jump(Rvalue::Constant(next),Guard::always());
        true
    })
}

pub fn binary_vv(opcode: &'static str,
                  decodea: fn(&mut State<Amd64>) -> Rvalue,
                  decodeb: fn(&mut State<Amd64>) -> Rvalue,
                  sem: fn(&mut CodeGen<Amd64>, Rvalue, Rvalue)
                 ) -> Box<Fn(&mut State<Amd64>) -> bool> {
    Box::new(move |st: &mut State<Amd64>| -> bool {
        let len = st.tokens.len();
        let next = st.address + len as u64;
        let arg0 = decodea(st);
        let arg1 = decodeb(st);

        st.mnemonic_dynargs(len,&opcode,"{64}, {64}",&|c| {
            sem(c,arg0.clone(),arg1.clone());
            vec![arg0.clone(),arg1.clone()]
        });
        st.jump(Rvalue::Constant(next),Guard::always());
        true
    })
}

pub fn trinary(opcode: &'static str,
              decode: fn(&mut State<Amd64>) -> (Rvalue,Rvalue,Rvalue),
              sem: fn(&mut CodeGen<Amd64>, Rvalue, Rvalue, Rvalue)
             ) -> Box<Fn(&mut State<Amd64>) -> bool> {
    Box::new(move |st: &mut State<Amd64>| -> bool {
        let len = st.tokens.len();
        let next = st.address + len as u64;
        let (arg0,arg1,arg2) = decode(st);

        st.mnemonic_dynargs(len,&opcode,"{64}, {64}, {64}",&|c| {
            sem(c,arg0.clone(),arg1.clone(),arg2.clone());
            vec![arg0.clone(),arg1.clone(),arg2.clone()]
        });
        st.jump(Rvalue::Constant(next),Guard::always());
        true
    })
}

pub fn trinary_vr(opcode: &'static str,
                  decode: fn(&mut State<Amd64>) -> (Rvalue,Rvalue),
                  c: &Lvalue,
                  sem: fn(&mut CodeGen<Amd64>, Rvalue, Rvalue, Rvalue)) -> Box<Fn(&mut State<Amd64>) -> bool> {
    let arg2 = c.clone();
    Box::new(move |st: &mut State<Amd64>| -> bool {
        let len = st.tokens.len();
        let next = st.address + len as u64;
        let (arg0,arg1) = decode(st);

        st.mnemonic_dynargs(len,&opcode,"{64}, {64}, {64}",&|c| {
            sem(c,arg0.clone(),arg1.clone(),arg2.to_rv());
            vec![arg0.clone(),arg1.clone(),arg2.to_rv()]
        });
        st.jump(Rvalue::Constant(next),Guard::always());
        true
    })
}

macro_rules! reg {
    ( $a:ident,$I:expr ) => {
        pub fn $a(st: &mut State<Amd64>) -> Rvalue {
            let r = if st.has_group("b") && st.get_group("b") == 1 { 8 } else { 0 } + $I;
            select_reg(&st.configuration.operand_size,r,st.configuration.rex).to_rv()
        }
    }
}

macro_rules! regd {
    ( $a:ident,$I:expr ) => {
        pub fn $a(st: &mut State<Amd64>) -> Rvalue {
            let r = if st.has_group("b") && st.get_group("b") == 1 { 8 } else { 0 } + $I;
            let opsz = if st.configuration.mode == Mode::Long && st.configuration.operand_size == OperandSize::ThirtyTwo {
                OperandSize::SixtyFour
            } else {
                st.configuration.operand_size
            };
            select_reg(&opsz,r,st.configuration.rex).to_rv()
        }
    }
}

macro_rules! regb {
    ( $a:ident,$I:expr ) => {
        pub fn $a(st: &mut State<Amd64>) -> Rvalue {
            let r = if st.has_group("b") && st.get_group("b") == 1 { 8 } else { 0 } + $I;
            select_reg(&OperandSize::Eight,r,st.configuration.rex).to_rv()
        }
    }
}

reg!(reg_a,0);
reg!(reg_c,1);
reg!(reg_d,2);
reg!(reg_b,3);
reg!(reg_sp,4);
reg!(reg_bp,5);
reg!(reg_si,6);
reg!(reg_di,7);

regd!(regd_a,0);
regd!(regd_c,1);
regd!(regd_d,2);
regd!(regd_b,3);
regd!(regd_sp,4);
regd!(regd_bp,5);
regd!(regd_si,6);
regd!(regd_di,7);

regb!(regb_a,0);
regb!(regb_c,1);
regb!(regb_d,2);
regb!(regb_b,3);
regb!(regb_sp,4);
regb!(regb_bp,5);
regb!(regb_si,6);
regb!(regb_di,7);

/*
sem_action po::amd64::nonary(std::string const& op, std::function<void(cg&)> func)
{
    return [op,func](sm &st)
    {
        st.mnemonic(st.tokens.size(),op,"",[func,st,op](cg& c)
        {
            func(c);

            std::cout << op << std::endl;
            return std::list<rvalue>({});
        });
        st.jump(st.address + st.tokens.size());
        return true;
    };
}

sem_action po::amd64::unary(std::string const& op, std::function<rvalue(sm const&,cg&)> decode, std::function<void(cg&,rvalue)> func)
{
    return [op,func,decode](sm &st)
    {
        st.mnemonic(st.tokens.size(),op,"{64}",[decode,func,st,op](cg& c)
        {
            rvalue a = decode(st,c);
            func(c,a);

            std::cout << "[ ";
            for(auto x: st.tokens)
                std::cout << std::setw(2) << std::hex << (unsigned int)x << " ";
            std::cout << "] " << op << " " << a << std::endl;
            return std::list<rvalue>({a});
        });
        st.jump(st.address + st.tokens.size());
        return true;
    };
}

sem_action po::amd64::unary(std::string const& op, rvalue arg, std::function<void(cg&,rvalue)> func)
{
    return [op,func,arg](sm &st)
    {
        st.mnemonic(st.tokens.size(),op,"{64}",[arg,func,st,op](cg& c)
        {
            func(c,arg);

            std::cout << "[ ";
            for(auto x: st.tokens)
                std::cout << std::setw(2) << std::hex << (unsigned int)x << " ";
            std::cout << "] " << op << " " << arg << std::endl;
            return std::list<rvalue>({arg});
        });
        st.jump(st.address + st.tokens.size());
        return true;
    };
}

sem_action po::amd64::binary(std::string const& op, std::function<std::pair<rvalue,rvalue>(sm const&,cg&)> decode, std::function<void(cg&,rvalue,rvalue)> func)
{
    return [op,func,decode](sm &st)
    {
        st.mnemonic(st.tokens.size(),op,"{64} {64}",[decode,func,st,op](cg& c)
        {
            rvalue a,b;

            std::tie(a,b) = decode(st,c);
            func(c,a,b);
            std::cout << "[ ";
            for(auto x: st.tokens)
                std::cout << std::setw(2) << std::hex << (unsigned int)x << " ";
            std::cout << "] " << op << " " << a << ", " << b << std::endl;
            return std::list<rvalue>({a,b});
        });

        st.jump(st.address + st.tokens.size());
        return true;
    };
}

sem_action po::amd64::binary(std::string const& op, std::function<rvalue(sm const&,cg&)> decode, rvalue arg2, std::function<void(cg&,rvalue,rvalue)> func)
{
    return [op,func,decode,arg2](sm &st)
    {
        st.mnemonic(st.tokens.size(),op,"{64} {64}",[arg2,decode,func,st,op](cg& c)
        {
            rvalue arg1 = decode(st,c);
            func(c,arg1,arg2);
            std::cout << "[ ";
            for(auto x: st.tokens)
                std::cout << std::setw(2) << std::hex << (unsigned int)x << " ";
            std::cout << "] " << op << " " << arg1 << ", " << arg2 << std::endl;
            return std::list<rvalue>({arg1,arg2});
        });

        st.jump(st.address + st.tokens.size());
        return true;
    };
}

sem_action po::amd64::binary(std::string const& op, rvalue arg1, std::function<rvalue(sm const&,cg&)> decode, std::function<void(cg&,rvalue,rvalue)> func)
{
    return [op,func,arg1,decode](sm &st)
    {
        st.mnemonic(st.tokens.size(),op,"{64} {64}",[arg1,decode,func,st,op](cg& c)
        {
            rvalue arg2 = decode(st,c);
            func(c,arg1,arg2);
            std::cout << "[ ";
            for(auto x: st.tokens)
                std::cout << std::setw(2) << std::hex << (unsigned int)x << " ";
            std::cout << "] " << op << " " << arg1 << ", " << arg2 << std::endl;
            return std::list<rvalue>({arg1,arg2});
        });

        st.jump(st.address + st.tokens.size());
        return true;
    };
}

sem_action po::amd64::binary(std::string const& op, rvalue arg1, rvalue arg2, std::function<void(cg&,rvalue,rvalue)> func)
{
    return [op,func,arg1,arg2](sm &st)
    {
        st.mnemonic(st.tokens.size(),op,"{64} {64}",[arg1,arg2,func,st,op](cg& c)
        {
            func(c,arg1,arg2);
            std::cout << "[ ";
            for(auto x: st.tokens)
                std::cout << std::setw(2) << std::hex << (unsigned int)x << " ";
            std::cout << "] " << op << " " << arg1 << ", " << arg2 << std::endl;
            return std::list<rvalue>({arg1,arg2});
        });

        st.jump(st.address + st.tokens.size());
        return true;
    };
}

sem_action po::amd64::binary(std::string const& op, std::function<rvalue(sm const&,cg&)> decode1, std::function<rvalue(sm const&,cg&)> decode2, std::function<void(cg&,rvalue,rvalue)> func)
{
    return [op,func,decode1,decode2](sm &st)
    {
        st.mnemonic(st.tokens.size(),op,"{64} {64}",[decode1,decode2,func,st,op](cg& c)
        {
            rvalue arg1 = decode1(st,c);
            rvalue arg2 = decode2(st,c);
            func(c,arg1,arg2);
            std::cout << "[ ";
            for(auto x: st.tokens)
                std::cout << std::setw(2) << std::hex << (unsigned int)x << " ";
            std::cout << "] " << op << " " << arg1 << ", " << arg2 << std::endl;
            return std::list<rvalue>({arg1,arg2});
        });

        st.jump(st.address + st.tokens.size());
        return true;
    };
}

sem_action po::amd64::branch(std::string const& m, rvalue flag, bool set)
{
    return [m,flag,set](sm &st)
    {
        /*int64_t _k = st.capture_groups["k"] * 2;
        guard g(flag,relation::Eq,set ? Rvalue::Constant(1) : Rvalue::Constant(0));
        Rvalue::Constant k((int8_t)(_k <= 63 ? _k : _k - 128));*/

        st.mnemonic(st.tokens.size() * 2,m,"");
        st.jump(st.address + st.tokens.size());//,g.negation());
        //st.jump(undefined(),g);//st.address + k.content() + 2,g);
        return true;
    };
}

sem_action po::amd64::trinary(std::string const& op, std::function<std::tuple<rvalue,rvalue,rvalue>(sm const&,cg&)> decode, std::function<void(cg&,rvalue,rvalue,rvalue)> func)
{
    return [op,func,decode](sm &st)
    {
        st.mnemonic(st.tokens.size(),op,"{64} {64} {64}",[decode,func,st,op](cg& d)
        {
            rvalue a,b,c;

            std::tie(a,b,c) = decode(st,d);
            func(d,a,b,c);

            std::cout << op << " " << a << ", " << b << ", " << c << std::endl;
            return std::list<rvalue>({a,b,c});
        });

        st.jump(st.address + st.tokens.size());
        return true;
    };
}

sem_action po::amd64::trinary(std::string const& op, std::function<std::pair<rvalue,rvalue>(sm const&,cg&)> decode, rvalue arg3, std::function<void(cg&,rvalue,rvalue,rvalue)> func)
{
    return [op,func,decode,arg3](sm &st)
    {
        st.mnemonic(st.tokens.size(),op,"{64} {64} {64}",[decode,arg3,func,st,op](cg& d)
        {
            rvalue a,b;

            std::tie(a,b) = decode(st,d);
            func(d,a,b,arg3);

            std::cout << op << " " << a << ", " << b << ", " << arg3 << std::endl;
            return std::list<rvalue>({a,b,arg3});
        });

        st.jump(st.address + st.tokens.size());
        return true;
    };
}
*/
