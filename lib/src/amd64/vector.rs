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

use disassembler::*;
use codegen::*;
use value::*;
use amd64::decode::*;
use amd64::semantic::*;
use amd64::*;

use std::rc::Rc;

pub fn mmx(rm0: Rc<Disassembler<Amd64>>, rm1: Rc<Disassembler<Amd64>>, rm2: Rc<Disassembler<Amd64>>,
           rm3: Rc<Disassembler<Amd64>>, rm4: Rc<Disassembler<Amd64>>, rm5: Rc<Disassembler<Amd64>>,
           rm6: Rc<Disassembler<Amd64>>, rm7: Rc<Disassembler<Amd64>>,
           rm: Rc<Disassembler<Amd64>>, imm8: Rc<Disassembler<Amd64>>) -> Rc<Disassembler<Amd64>> {
    new_disassembler!(Amd64 =>
        // EMMS
        [ 0x0f, 0x77 ] = nonary("emms",emms),

        // PACKSS*
        [ 0x0f, 0x63, rm ] = binary("packsswb",decode_rm,packsswb),
        [ 0x0f, 0x6b, rm ] = binary("packssdw",decode_rm,packssdw),

        // PACKUSWB
        [ 0x0f, 0x67, rm ] = binary("packuswb",decode_rm,packuswb),

        // PADD*
        [ 0x0f, 0xfc, rm ] = binary("paddb",decode_rm,paddb),
        [ 0x0f, 0xfd, rm ] = binary("paddw",decode_rm,paddw),
        [ 0x0f, 0xfe, rm ] = binary("paddd",decode_rm,paddd),

        // PADDS*
        [ 0x0f, 0xec, rm ] = binary("paddsb",decode_rm,paddsb),
        [ 0x0f, 0xed, rm ] = binary("paddsw",decode_rm,paddsw),

        // PADDUS*
        [ 0x0f, 0xdc, rm ] = binary("paddusb",decode_rm,paddusb),
        [ 0x0f, 0xdd, rm ] = binary("paddusw",decode_rm,paddusw),

        // PAND
        [ 0x0f, 0xdb, rm ] = binary("pand",decode_rm,pand),

        // PANDN
        [ 0x0f, 0xdf, rm ] = binary("pandn",decode_rm,pandn),

        // PCMPEQ*
        [ 0x0f, 0x74, rm ] = binary("pcmpeqb",decode_rm,pcmpeqb),
        [ 0x0f, 0x75, rm ] = binary("pcmpeqw",decode_rm,pcmpeqw),
        [ 0x0f, 0x76, rm ] = binary("pcmpeqd",decode_rm,pcmpeqd),

        // PCMPGT*
        [ 0x0f, 0x64, rm ] = binary("pcmpgtb",decode_rm,pcmpgtb),
        [ 0x0f, 0x65, rm ] = binary("pcmpgtw",decode_rm,pcmpgtw),
        [ 0x0f, 0x66, rm ] = binary("pcmpgtd",decode_rm,pcmpgtd),

        // PMADDWD
        [ 0x0f, 0xf5, rm ] = binary("pmadwd",decode_rm,pmadwd),

        // PMULHW
        [ 0x0f, 0xe5, rm ] = binary("pmulhw",decode_rm,pmulhw),

        // PMULLW
        [ 0x0f, 0xd5, rm ] = binary("pmullw",decode_rm,pmullw),

        // POR
        [ 0x0f, 0xeb, rm ] = binary("por",decode_rm,por),

        // PSLLW
        [ 0x0f, 0xf1, rm        ] = binary("psllw",decode_rm,psllw),
        [ 0x0f, 0x71, rm6, imm8 ] = binary("psllw",decode_mi,psllw),

        // PSLLD
        [ 0x0f, 0xf2, rm        ] = binary("pslld",decode_rm,pslld),
        [ 0x0f, 0x72, rm6, imm8 ] = binary("pslld",decode_mi,pslld),

        // PSLLQ
        [ 0x0f, 0xf3, rm        ] = binary("psllq",decode_rm,psllq),
        [ 0x0f, 0x73, rm6, imm8 ] = binary("psllq",decode_mi,psllq),

        // PSRAW
        [ 0x0f, 0xe1, rm        ] = binary("psraw",decode_rm,psraw),
        [ 0x0f, 0x71, rm4, imm8 ] = binary("psraw",decode_mi,psraw),

        // PSRAD
        [ 0x0f, 0xe2, rm        ] = binary("psrad",decode_rm,psrad),
        [ 0x0f, 0x72, rm4, imm8 ] = binary("psrad",decode_mi,psrad),

        // PSRLW
        [ 0x0f, 0xd1, rm        ] = binary("psrlw",decode_rm,psrlw),
        [ 0x0f, 0x71, rm2, imm8 ] = binary("psrlw",decode_mi,psrlw),

        // PSRLD
        [ 0x0f, 0xd2, rm        ] = binary("psrld",decode_rm,psrld),
        [ 0x0f, 0x71, rm2, imm8 ] = binary("psrld",decode_mi,psrld),

        // PSRLQ
        [ 0x0f, 0xd3, rm        ] = binary("psrlq",decode_rm,psrlq),
        [ 0x0f, 0x71, rm2, imm8 ] = binary("psrlq",decode_mi,psrlq),

        // PSUB*
        [ 0x0f, 0xf8, rm ] = binary("psubb",decode_rm,psubb),
        [ 0x0f, 0xf9, rm ] = binary("psubw",decode_rm,psubw),
        [ 0x0f, 0xfa, rm ] = binary("psubd",decode_rm,psubd),

        // PSUBS*
        [ 0x0f, 0xe8, rm ] = binary("psubsb",decode_rm,psubsb),
        [ 0x0f, 0xe9, rm ] = binary("psubsw",decode_rm,psubsw),

        // PSUBUS*
        [ 0x0f, 0xd8, rm ] = binary("psubusb",decode_rm,psubusb),
        [ 0x0f, 0xd9, rm ] = binary("psubusw",decode_rm,psubusw),

        // PUNPCKH*
        [ 0x0f, 0x68, rm ] = binary("punpckhbw",decode_rm,punpckhbw),
        [ 0x0f, 0x69, rm ] = binary("punpckhwd",decode_rm,punpckhwd),
        [ 0x0f, 0x6a, rm ] = binary("punpckhdq",decode_rm,punpckhdq),

        // PUNPCKL*
        [ 0x0f, 0x60, rm ] = binary("punpcklbw",decode_rm,punpcklbw),
        [ 0x0f, 0x61, rm ] = binary("punpcklwd",decode_rm,punpcklwd),
        [ 0x0f, 0x62, rm ] = binary("punpckldq",decode_rm,punpckldq),

        // PXOR
        [ 0x0f, 0xef, rm ] = binary("pxor",decode_rm,pxor),

        // MOVD/MOVQ
        [ 0x0f, 0x6e, rm ] = binary("movd",decode_rm,mov),
        [ 0x0f, 0x7e, rm ] = binary("movd",decode_mr,mov))
}

pub fn sse1(rm0: Rc<Disassembler<Amd64>>, rm1: Rc<Disassembler<Amd64>>, rm2: Rc<Disassembler<Amd64>>,
            rm3: Rc<Disassembler<Amd64>>, rm4: Rc<Disassembler<Amd64>>, rm5: Rc<Disassembler<Amd64>>,
            rm6: Rc<Disassembler<Amd64>>, rm7: Rc<Disassembler<Amd64>>,
            rm: Rc<Disassembler<Amd64>>, imm8: Rc<Disassembler<Amd64>>,
            rexw_prfx: Rc<Disassembler<Amd64>>) -> Rc<Disassembler<Amd64>> {
    new_disassembler!(Amd64 =>
        // ADDPS
        [ 0x0f, 0x58, rm ] = binary("addps",decode_rm,addps),

        // ADDSS
        [ 0xf3, 0x0f, 0x58, rm ] = binary("addss",decode_rm,addss),

        // ANDNPS
        [ 0x0f, 0x55, rm ] = binary("andnps",decode_rm,andnps),

        // ANDPS
        [ 0x0f, 0x54, rm ] = binary("andps",decode_rm,andps),

        // CMPPS
        [ 0x0f, 0xc2, rm, imm8 ] = trinary("cmpps",decode_rmi,cmpps),

        // CMPSS
        [ 0xf3, 0x0f, 0xc2, rm, imm8 ] = trinary("cmpss",decode_rmi,cmpss),

        // COMISS
        [ 0x0f, 0x2f, rm ] = binary("comiss",decode_rm,comiss),

        // CVTPI2PS
        [ 0x0f, 0x2a, rm ] = binary("cvtpi2ps",decode_rm,cvtpi2ps),

        // CVTPS2PI
        [ 0x0f, 0x2d, rm ] = binary("cvtps2pi",decode_rm,cvtps2pi),

        // CVTSI2SS
        [ 0xf3, 0x0f, 0x2a, rm ] = binary("cvtsi2ss",decode_rm,cvtsi2ss),

        // CVTSS2SI
        [ 0xf3, 0x0f, 0x2d, rm ] = binary("cvtss2si",decode_rm,cvtss2si),

        // CVTTPS2PI
        [ 0x0f, 0x2c, rm ] = binary("cvttps2pi",decode_rm,cvttps2pi),

        // CVTTSS2SI
        [ 0xf3, opt!(rexw_prfx), 0x0f, 0x2c, rm ] = binary("cvttss2si",decode_rm,cvttss2si),

        // DIV*S
        [ 0x0f, 0x5e, rm ] = binary("divps",decode_rm,divps),
        [ 0xf3, 0x0f, 0x5e, rm ] = binary("divss",decode_rm,divss),

        // LDMXCSR
        [ 0x0f, 0xae, rm2 ] = binary("ldmxcsr",decode_rm,ldmxcsr),

        // MASKMOVQ
        [ 0x0f, 0xf7, rm ] = binary("maskmovq",decode_rm,maskmovq),

        // MAX*S
        [ 0x0f, 0x5f, rm ] = binary("maxps",decode_rm,maxps),
        [ 0xf3, 0x0f, 0x5f, rm ] = binary("maxss",decode_rm,maxss),

        // MIN*S
        [ 0x0f, 0x5d, rm ] = binary("minps",decode_rm,minps),
        [ 0xf3, 0x0f, 0x5d, rm ] = binary("minss",decode_rm,minss),

        // MOVAPS
        [ 0x0f, 0x28, rm ] = binary("movaps",decode_rm,movaps),
        [ 0x0f, 0x29, rm ] = binary("movaps",decode_mr,movaps),

        // MOVHPS
        [ 0x0f, 0x16, rm ] = binary("minhps",decode_rm,minhps),
        [ 0x0f, 0x17, rm ] = binary("minhps",decode_mr,minhps),

        // MOVLPS
        [ 0x0f, 0x12, rm ] = binary("movlps",decode_rm,movlps),
        [ 0x0f, 0x13, rm ] = binary("movlps",decode_mr,movlps),

        // MOVMSKPS
        [ 0x0f, 0x50, rm ] = binary("movmskps",decode_rm,movmskps),

        // MOVNTPS
        [ 0x0f, 0x2b, rm ] = binary("movntps",decode_mr,movntps),

        // MOVNTQ
        [ 0x0f, 0xe7, rm ] = binary("movntq",decode_mr,movntq),

        // MOVSS
        [ 0xf3, 0x0f, 0x10, rm ] = binary("movss",decode_rm,movss),
        [ 0xf3, 0x0f, 0x11, rm ] = binary("movss",decode_mr,movss),

        // MOVUPS
        [ 0x0f, 0x10, rm ] = binary("movups",decode_rm,movups),
        [ 0x0f, 0x11, rm ] = binary("movups",decode_mr,movups),

        // MUL*S
        [ 0x0f, 0x59, rm ] = binary("mulps",decode_rm,mulps),
        [ 0xf3, 0x0f, 0x59, rm ] = binary("mulss",decode_rm,mulss),

        // ORPS
        [ 0x0f, 0x56, rm ] = binary("orps",decode_rm,orps),

        // PAVG*
        [ 0x0f, 0xe0, rm ] = binary("pavgb",decode_rm,pavgb),
        [ 0x0f, 0xe3, rm ] = binary("pavgw",decode_rm,pavgw),

        // PEXTRW
        [ 0x0f, 0xc5, rm, imm8 ] = trinary("pextrw",decode_rmi,pextrw),

        // PINSRW
        [ 0x0f, 0xc4, rm, imm8 ] = trinary("pinsrw",decode_rmi,pinsrw),

        // PMAX*
        [ 0x0f, 0xee, rm ] = binary("pmaxsw",decode_rm,pmaxsw),
        [ 0x0f, 0xde, rm ] = binary("pmaxub",decode_rm,pmaxub),

        // PMIN*
        [ 0x0f, 0xea, rm ] = binary("pminsw",decode_rm,pminsw),
        [ 0x0f, 0xda, rm ] = binary("pminub",decode_rm,pminub),

        // PMOVMSKB
        [ 0x0f, 0xd7, rm ] = binary("pmovmskb",decode_rm,pmovmskb),

        // PMULHUW
        [ 0x0f, 0xe4, rm ] = binary("pmulhuw",decode_rm,pmulhuw),

        // PREFETCH*
        [ 0x0f, 0x18, rm0 ] = unary("prefetchnta",decode_m,prefetchnta),
        [ 0x0f, 0x18, rm1 ] = unary("prefetcht0",decode_m,prefetcht0),
        [ 0x0f, 0x18, rm2 ] = unary("prefetcht1",decode_m,prefetcht1),
        [ 0x0f, 0x18, rm3 ] = unary("prefetcht2",decode_m,prefetcht2),

        // PSADBW
        [ 0x0f, 0xf6, rm ] = binary("psadbw",decode_rm,psadbw),

        // PSHUFW
        [ 0x0f, 0x70, rm, imm8 ] = trinary("pshufw",decode_rmi,pshufw),

        // RCP*S
        [ 0x0f, 0x53, rm ] = binary("rcpps",decode_rm,rcpps),
        [ 0xf3, 0x0f, 0x53, rm ] = binary("rcpss",decode_rm,rcpss),

        // RSQRT*S
        [ 0x0f, 0x52, rm ] = binary("rsqrtps",decode_rm,rsqrtps),
        [ 0xf3, 0x0f, 0x52, rm ] = binary("rsqrtss",decode_rm,rsqrtss),

        // SFENCE
        [ 0x0f, 0xae, 0xf8 ] = nonary("sfence",sfence),

        // SHUFPS
        [ 0x0f, 0xc6, rm, imm8 ] = trinary("shufps",decode_rmi,shufps),

        // SQRT*S
        [ 0x0f, 0x51, rm ] = binary("sqrtps",decode_rm,sqrtps),
        [ 0xf3, 0x0f, 0x51, rm ] = binary("sqrtss",decode_rm,sqrtss),

        // STMXCSR
        [ 0x0f, 0xae, rm3 ] = unary("stmxcsr",decode_m,stmxcsr),

        // SUB*S
        [ 0x0f, 0x5c, rm ] = binary("subps",decode_rm,subps),
        [ 0xf3, 0x0f, 0x5c, rm ] = binary("subss",decode_rm,subss),

        // UCOMISS
        [ 0x0f, 0x2e, rm ] = binary("ucomiss",decode_rm,ucomiss),

        // UNPCK*PS
        [ 0x0f, 0x15, rm ] = binary("unpckhps",decode_rm,unpckhps),
        [ 0x0f, 0x14, rm ] = binary("unpcklps",decode_rm,unpcklps),

        // XORPS
        [ 0x0f, 0x57, rm ] = binary("unpckhps",decode_rm,xorps))
}

pub fn sse2(rm: Rc<Disassembler<Amd64>>) -> Rc<Disassembler<Amd64>> {
    new_disassembler!(Amd64 =>
        // MOVAPD
        [ 0x66, 0x0f, 0x28, rm ] = binary("movapd",decode_rm,movapd),
        [ 0x66, 0x0f, 0x28, rm ] = binary("movapd",decode_mr,movapd),

        // MOVD/MOVQ
        [ 0x66, 0x0f, 0x6e, rm ] = binary("movd",decode_rm,mov),
        [ 0x66, 0x0f, 0x7e, rm ] = binary("movd",decode_mr,mov))
}

pub fn avx(vex_prfx: Rc<Disassembler<Amd64>>, rm: Rc<Disassembler<Amd64>>) -> Rc<Disassembler<Amd64>> {
    new_disassembler!(Amd64 =>
        // VZEROUPPER
        [ vex_prfx, 0x77 ] = nonary("vzeroupper",vzeroupper),

        // MOVD/MOVQ
        [ vex_prfx, 0x6e, rm ] = binary("movd",decode_rm,mov),
        [ vex_prfx, 0x7e, rm ] = binary("movd",decode_rm,mov),

        // MOVAPD
        [ vex_prfx, 0x28, rm ] = binary("movapd",decode_rm,movapd),
        [ vex_prfx, 0x29, rm ] = binary("movapd",decode_mr,movapd))
}
