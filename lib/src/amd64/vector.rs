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

        [ 0x0f, 0x0d, rm1 ] = unary("prefetchw",decode_m,prefetchw),
        [ 0x0f, 0x0d, rm2 ] = unary("prefetchwt1",decode_m,prefetchwt1),

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

pub fn sse2(rm0: Rc<Disassembler<Amd64>>, rm1: Rc<Disassembler<Amd64>>, rm2: Rc<Disassembler<Amd64>>,
            rm3: Rc<Disassembler<Amd64>>, rm4: Rc<Disassembler<Amd64>>, rm5: Rc<Disassembler<Amd64>>,
            rm6: Rc<Disassembler<Amd64>>, rm7: Rc<Disassembler<Amd64>>,
            rm: Rc<Disassembler<Amd64>>, imm8: Rc<Disassembler<Amd64>>,
            rex_prfx: Rc<Disassembler<Amd64>>, rexw_prfx: Rc<Disassembler<Amd64>>) -> Rc<Disassembler<Amd64>> {
    new_disassembler!(Amd64 =>
        // MOVAPD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x28, rm ] = binary("movapd",decode_rm,movapd),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x28, rm ] = binary("movapd",decode_mr,movapd),

        // ADD*D
        [ 0x66, opt!(rex_prfx), 0x0f, 0x58, rm ] = binary("addpd",decode_rm,addpd),
        [ 0xf2, opt!(rex_prfx), 0x0f, 0x58, rm ] = binary("addsd",decode_rm,addsd),

        // ANDNPD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x52, rm ] = binary("addpd",decode_rm,andnpd),

        // ANDPD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x54, rm ] = binary("addpd",decode_rm,andpd),

        // CLFLUSH
        [ 0x0f, 0xad, rm7 ] = unary("addpd",decode_m,cflush),

        // CMP*D
        [ 0x66, opt!(rex_prfx), 0x0f, 0xc2, rm, imm8 ] = trinary("cmppd",decode_rmi,cmppd),
        [ 0xf2, opt!(rex_prfx), 0x0f, 0xc2, rm, imm8 ] = trinary("cmpsd",decode_rmi,cmpsd),

        // COMISD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x2f, rm ] = binary("comisd",decode_rm,comisd),

        // CVTDQ2PD
        [ 0xf3, opt!(rex_prfx), 0x0f, 0xe6, rm ] = binary("cvtdq2pd",decode_rm,cvtdq2pd),

        // CVTDQ2PS
        [ 0x0f, 0x5b, rm ] = binary("cvtdq2ps",decode_rm,cvtdq2ps),

        // CVTPD2DQ
        [ 0xf2, opt!(rex_prfx), 0x0f, 0xe6, rm ] = binary("cvtdq2pd",decode_rm,cvtpd2dq),

        // CVTPD2PI
        [ 0x66, opt!(rex_prfx), 0x0f, 0x2d, rm ] = binary("cvtpd2pi",decode_rm,cvtpd2pi),

        // CVTPD2PS
        [ 0x66, opt!(rex_prfx), 0x0f, 0x5a, rm ] = binary("cvtpd2ps",decode_rm,cvtpd2ps),

        // CVTPI2PD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x2a, rm ] = binary("cvtpi2pd",decode_rm,cvtpi2pd),

        // CVTPS2DQ
        [ 0x66, opt!(rex_prfx), 0x0f, 0x5b, rm ] = binary("cvtps2dq",decode_rm,cvtps2dq),

        // CVTPS2PD
        [ 0x0f, 0x5a, rm ] = binary("cvtps2pd",decode_rm,cvtps2pd),

        // CVTSD2SI
        [ 0xf2, opt!(rexw_prfx), 0x0f, 0x2d, rm ] = binary("cvtsd2si",decode_rm,cvtsd2si),

        // CVTSD2SS
        [ 0xf2, opt!(rex_prfx), 0x0f, 0x5a, rm ] = binary("cvtsd2ss",decode_rm,cvtsd2ss),

        // CVTSI2SD
        [ 0xf2, opt!(rexw_prfx), 0x0f, 0x2a, rm ] = binary("cvtsi2sd",decode_rm,cvtsi2sd),

        // CVTSS2SD
        [ 0xf3, opt!(rex_prfx), 0x0f, 0x5a, rm ] = binary("cvtss2sd",decode_rm,cvtss2sd),

        // CVTTPD2DQ
        [ 0x66, opt!(rex_prfx), 0x0f, 0xe6, rm ] = binary("cvttpd2dq",decode_rm,cvttpd2dq),

        // CVTTPD2PI
        [ 0x66, opt!(rex_prfx), 0x0f, 0x2c, rm ] = binary("cvttpd2pi",decode_rm,cvttpd2pi),

        // CVTTPS2DQ
        [ 0xf3, opt!(rex_prfx), 0x0f, 0x5b, rm ] = binary("cvttps2dq",decode_rm,cvttps2dq),

        // CVTTSD2SI
        [ 0xf2, opt!(rexw_prfx), 0x0f, 0x2c, rm ] = binary("cvttsd2si",decode_rm,cvttsd2si),

        // DIV*D
        [ 0x66, opt!(rex_prfx), 0x0f, 0x5e, rm ] = binary("divpd",decode_rm,divpd),
        [ 0xf2, opt!(rex_prfx), 0x0f, 0x5e, rm ] = binary("divsd",decode_rm,divsd),

        // LFENCE
        [ 0x0f, 0xae, 0xe8 ] = nonary("lfence",lfence),

        // MASKMOVDQU
        [ 0x66, opt!(rex_prfx), 0x0f, 0xf7, rm ] = binary("maskmovdqu",decode_rm,maskmovdqu),

        // MAX*D
        [ 0x66, opt!(rex_prfx), 0x0f, 0x5f, rm ] = binary("maxpd",decode_rm,maxpd),
        [ 0xf2, opt!(rex_prfx), 0x0f, 0x5f, rm ] = binary("maxsd",decode_rm,maxsd),

        // MFENCE
        [ 0x0f, 0xae, 0xf0 ] = nonary("mfence",mfence),

        // MIN*D
        [ 0x66, opt!(rex_prfx), 0x0f, 0x5d, rm ] = binary("minpd",decode_rm,minpd),
        [ 0xf2, opt!(rex_prfx), 0x0f, 0x5d, rm ] = binary("minsd",decode_rm,minsd),

        // MOVAPD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x28, rm ] = binary("movapd",decode_rm,movapd),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x29, rm ] = binary("movapd",decode_mr,movapd),

        // MOVD
        [ 0x66, opt!(rexw_prfx), 0x0f, 0x6e, rm ] = binary("movd",decode_rm,movd),
        [ 0x66, opt!(rexw_prfx), 0x0f, 0x7e, rm ] = binary("movd",decode_mr,movd),

        // MOVDQ2Q
        [ 0xf2, opt!(rex_prfx), 0x0f, 0xd6, rm ] = binary("movdq2q",decode_rm,movdq2q),

        // MOVDQA
        [ 0x66, opt!(rex_prfx), 0x0f, 0x6f, rm ] = binary("movdqa",decode_rm,movdaq),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x7f, rm ] = binary("movdqa",decode_mr,movdqa),

        // MOVDQU
        [ 0xf3, opt!(rex_prfx), 0x0f, 0x6f, rm ] = binary("movdqu",decode_rm,movdqu),
        [ 0xf3, opt!(rex_prfx), 0x0f, 0x7f, rm ] = binary("movdqu",decode_mr,movdqu),

        // MOVHPD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x16, rm ] = binary("movhpd",decode_rm,movhpd),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x17, rm ] = binary("movhpd",decode_mr,movhpd),

        // MOVLPD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x12, rm ] = binary("movlpd",decode_rm,movlpd),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x13, rm ] = binary("movlpd",decode_mr,movlpd),

        // MOVMSKPD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x50, rm ] = binary("movmskpd",decode_rm,movmskpd),

        // MOVNTDQ
        [ 0x66, opt!(rex_prfx), 0x0f, 0xe7, rm ] = binary("movntdq",decode_mr,movntdq),

        // MOVNTI
        [ opt!(rexw_prfx), 0x0f, 0xc3, rm ] = binary("movapd",decode_mr,movnti),

        // MOVNTPD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x2b, rm ] = binary("movntpd",decode_mr,movntpd),

        // MOVQ
        [ 0xf3, opt!(rex_prfx), 0x0f, 0x7e, rm ] = binary("movq",decode_rm,movq),
        [ 0x66, opt!(rex_prfx), 0x0f, 0xd6, rm ] = binary("movq",decode_mr,movq),

        // MOVQ2DQ
        [ 0xf3, opt!(rex_prfx), 0x0f, 0xd6, rm ] = binary("movq2dq",decode_rm,movq2dq),

        // MOVSD
        [ 0xf2, opt!(rex_prfx), 0x0f, 0x10, rm ] = binary("movsd",decode_rm,movsd),
        [ 0xf2, opt!(rex_prfx), 0x0f, 0x11, rm ] = binary("movsd",decode_mr,movsd),

        // MOVUPD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x10, rm ] = binary("movupd",decode_rm,movupd),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x11, rm ] = binary("movupd",decode_mr,movupd),

        // MULPD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x59, rm ] = binary("mulpd",decode_rm,mulpd),

        // MULSD
        [ 0xf2, opt!(rex_prfx), 0x0f, 0x59, rm ] = binary("mulsd",decode_rm,mulsd),

        // ORPD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x56, rm ] = binary("orpd",decode_rm,orpd),

        // PACKSS*
        [ 0x66, opt!(rex_prfx), 0x0f, 0x63, rm ] = binary("packsswb",decode_rm,packsswb),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x6b, rm ] = binary("packssdw",decode_rm,packssdw),

        // PACKUSWB
        [ 0x66, opt!(rex_prfx), 0x0f, 0x67, rm ] = binary("packuswb",decode_rm,packuswb),

        // PADD*
        [ 0x66, opt!(rex_prfx), 0x0f, 0xfc, rm ] = binary("paddb",decode_rm,paddb),
        [ 0x66, opt!(rex_prfx), 0x0f, 0xfd, rm ] = binary("paddw",decode_rm,paddw),
        [ 0x66, opt!(rex_prfx), 0x0f, 0xfe, rm ] = binary("paddd",decode_rm,paddd),
        [ opt!(0x66), opt!(rex_prfx), 0x0f, 0xd4, rm ] = binary("paddq",decode_rm,paddq),

        // PADDS*
        [ 0x66, opt!(rex_prfx), 0x0f, 0xec, rm ] = binary("paddsb",decode_rm,paddsb),
        [ 0x66, opt!(rex_prfx), 0x0f, 0xed, rm ] = binary("paddsw",decode_rm,paddsw),

        // PADDUS*
        [ 0x66, opt!(rex_prfx), 0x0f, 0xdc, rm ] = binary("paddusb",decode_rm,paddusb),
        [ 0x66, opt!(rex_prfx), 0x0f, 0xdd, rm ] = binary("paddusw",decode_rm,paddusw),

        // PAND
        [ 0x66, opt!(rex_prfx), 0x0f, 0xdb, rm ] = binary("pand",decode_rm,pand),

        // PANDN
        [ 0x66, opt!(rex_prfx), 0x0f, 0xdf, rm ] = binary("pandn",decode_rm,pandn),

        // PAUSE
        [ 0xf3, 0x90 ] = nonary("pause",pause),

        // PCMPEQ*
        [ 0x66, opt!(rex_prfx), 0x0f, 0x74, rm ] = binary("pcmpeqb",decode_rm,pcmpeqb),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x75, rm ] = binary("pcmpeqw",decode_rm,pcmpeqw),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x76, rm ] = binary("pcmpeqd",decode_rm,pcmpeqd),

        // PCMPGT*
        [ 0x66, opt!(rex_prfx), 0x0f, 0x64, rm ] = binary("pcmpgtb",decode_rm,pcmpgtb),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x65, rm ] = binary("pcmpgtw",decode_rm,pcmpgtw),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x66, rm ] = binary("pcmpgtd",decode_rm,pcmpgtd),

        // PMADDWD
        [ 0x66, opt!(rex_prfx), 0x0f, 0xf5, rm ] = binary("pmaddwd",decode_rm,pmadwd),

        // PMUL*W
        [ 0x66, opt!(rex_prfx), 0x0f, 0xe5, rm ] = binary("pmulhw",decode_rm,pmulhw),
        [ 0x66, opt!(rex_prfx), 0x0f, 0xd5, rm ] = binary("pmullw",decode_rm,pmullw),

        // PMULUDQ
        [ opt!(0x66), opt!(rex_prfx), 0x0f, 0xf4, rm ] = binary("pcmpgtd",decode_rm,pmuludq),

        // POR
        [ 0x66, opt!(rex_prfx), 0x0f, 0xeb, rm ] = binary("pcmpgtd",decode_rm,por),

        // PSHUFD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x70, rm, imm8 ] = trinary("pshufd",decode_rmi,pshufd),

        // PSHUFHW
        [ 0xf3, opt!(rex_prfx), 0x0f, 0x70, rm, imm8 ] = trinary("pshufhw",decode_rmi,pshufhw),
        [ 0xf2, opt!(rex_prfx), 0x0f, 0x70, rm, imm8 ] = trinary("pshuflw",decode_rmi,pshuflw),

        // PSLLD
        [ 0x66, opt!(rex_prfx), 0x0f, 0xf1, rm        ] = binary("psllw",decode_rm,psllw),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x71, rm6, imm8 ] = binary("psllw",decode_mi,psllw),
        [ 0x66, opt!(rex_prfx), 0x0f, 0xf2, rm        ] = binary("pslld",decode_rm,pslld),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x72, rm6, imm8 ] = binary("pslld",decode_mi,pslld),
        [ 0x66, opt!(rex_prfx), 0x0f, 0xf3, rm        ] = binary("psllq",decode_rm,psllq),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x73, rm6, imm8 ] = binary("psllq",decode_mi,psllq),

        // PSLLDQ
        [ 0x66, opt!(rex_prfx), 0x0f, 0x73, rm7, imm8 ] = binary("pslldq",decode_mi,pslldq),

        // PSRAD
        [ 0x66, opt!(rex_prfx), 0x0f, 0xe2, rm        ] = binary("psrad",decode_rm,psrad),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x72, rm4, imm8 ] = binary("psrad",decode_mi,psrad),

        // PSRAW
        [ 0x66, opt!(rex_prfx), 0x0f, 0xe1, rm        ] = binary("psraw",decode_rm,psarw),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x71, rm4, imm8 ] = binary("psraw",decode_mi,psarw),

        // PSRLW
        [ 0x66, opt!(rex_prfx), 0x0f, 0xd1, rm        ] = binary("psrlw",decode_rm,psrlw),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x71, rm2, imm8 ] = binary("psrlw",decode_mi,psrlw),
        [ 0x66, opt!(rex_prfx), 0x0f, 0xd2, rm        ] = binary("psrld",decode_rm,psrld),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x72, rm2, imm8 ] = binary("psrld",decode_mi,psrld),
        [ 0x66, opt!(rex_prfx), 0x0f, 0xd3, rm        ] = binary("psrlq",decode_rm,psrlq),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x73, rm2, imm8 ] = binary("psrlq",decode_mi,psrlq),

        // PSRLDQ
        [ 0x66, opt!(rex_prfx), 0x0f, 0x73, rm3, imm8 ] = binary("psrldq",decode_mi,psrldq),

        // PSUBB
        [ 0x66, opt!(rex_prfx), 0x0f, 0xf8, rm ] = binary("psubb",decode_rm,psubb),
        [ 0x66, opt!(rex_prfx), 0x0f, 0xf9, rm ] = binary("psubw",decode_rm,psubw),
        [ 0x66, opt!(rex_prfx), 0x0f, 0xfa, rm ] = binary("psubd",decode_rm,psubd),
        [ opt!(0x66), opt!(rex_prfx), 0x0f, 0xfb, rm ] = binary("psubq",decode_rm,psubq),

        // PSUBS*
        [ 0x66, opt!(rex_prfx), 0x0f, 0xe8, rm ] = binary("psubsb",decode_rm,psubsb),
        [ 0x66, opt!(rex_prfx), 0x0f, 0xe9, rm ] = binary("psubsw",decode_rm,pusbsw),

        // PSUBUS*
        [ 0x66, opt!(rex_prfx), 0x0f, 0xd8, rm ] = binary("psubusb",decode_rm,psubusb),
        [ 0x66, opt!(rex_prfx), 0x0f, 0xd9, rm ] = binary("psubusw",decode_rm,psubusw),

        // PMOVMSKB
        [ 0x66, opt!(rex_prfx), opt!(rex_prfx), 0x0f, 0xd7, rm ] = binary("pmovmskb",decode_rm,pmovmskb),

        // PMAX*
        [ 0x66, opt!(rex_prfx), 0x0f, 0xee, rm ] = binary("pmaxsw",decode_rm,pmaxsw),
        [ 0x66, opt!(rex_prfx), 0x0f, 0xde, rm ] = binary("pmaxub",decode_rm,pmaxub),

        // PMIN*
        [ 0x66, opt!(rex_prfx), 0x0f, 0xea, rm ] = binary("pminsw",decode_rm,pminsw),
        [ 0x66, opt!(rex_prfx), 0x0f, 0xda, rm ] = binary("pminub",decode_rm,pminub),

        // PUNPCKHBW
        [ 0x66, opt!(rex_prfx), 0x0f, 0x68, rm ] = binary("punpckhbw",decode_rm,punpckhbw),

        // PUNPCKHWD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x69, rm ] = binary("punpckhwd",decode_rm,punckhwd),

        // PUNPCKHDQ
        [ 0x66, opt!(rex_prfx), 0x0f, 0x6a, rm ] = binary("punpckhwd",decode_rm,punpckhdq),

        // PUNPCKHQDQ
        [ 0x66, opt!(rex_prfx), 0x0f, 0x6d, rm ] = binary("punpckhqdq",decode_rm,punpckhqdq),

        // PUNPCKLBW
        [ 0x66, opt!(rex_prfx), 0x0f, 0x60, rm ] = binary("punpcklbw",decode_rm,punpcklbw),

        // PUNPCKLDQ
        [ 0x66, opt!(rex_prfx), 0x0f, 0x61, rm ] = binary("punpckldq",decode_rm,punpckldq),

        // PUNPCKLQDQ
        [ 0x66, opt!(rex_prfx), 0x0f, 0x62, rm ] = binary("punpcklqdq",decode_rm,puncklqdq),

        // PUNPCKLWD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x6c, rm ] = binary("punpcklwd",decode_rm,puncklwd),

        // PXOR
        [ 0x66, opt!(rex_prfx), 0x0f, 0xef, rm ] = binary("pxor",decode_rm,pxor),

        // SHUFPD
        [ 0x66, opt!(rex_prfx), 0x0f, 0xc6, rm, imm8 ] = trinary("shufpd",decode_rmi,shufpd),

        // SQRTPD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x51, rm ] = binary("sqrtpd",decode_rm,sqrtpd),

        // SQRTSD
        [ 0xf2, opt!(rex_prfx), 0x0f, 0x51, rm ] = binary("sqrtsd",decode_rm,sqrtsd),

        // SUBPD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x5c, rm ] = binary("subpd",decode_rm,subpd),

        // SUBSD
        [ 0xf2, opt!(rex_prfx), 0x0f, 0x5c, rm ] = binary("subsd",decode_rm,subsd),

        // UCOMISD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x2e, rm ] = binary("ucomisd",decode_rm,ucomisd),

        // UNPCKHPD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x15, rm ] = binary("unpckhpd",decode_rm,unpckhpd),

        // UNPCKLPD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x14, rm ] = binary("unpcklpd",decode_rm,unpcklpd),

        // XORPD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x57, rm ] = binary("xorpd",decode_rm,xorpd),

        // MOVD/MOVQ
        [ 0x66, opt!(rex_prfx), 0x0f, 0x6e, rm ] = binary("movd",decode_rm,mov),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x7e, rm ] = binary("movd",decode_mr,mov))
}

pub fn sse3(rm: Rc<Disassembler<Amd64>>, imm8: Rc<Disassembler<Amd64>>,
            rex_prfx: Rc<Disassembler<Amd64>>, rexw_prfx: Rc<Disassembler<Amd64>>) -> Rc<Disassembler<Amd64>> {
    new_disassembler!(Amd64 =>
        // ADDSUBPD
        [ 0x66, opt!(rex_prfx), 0x0f, 0xd0, rm ] = binary("addsubpd",decode_rm,addsubpd),

        // ADDSUBPS
        [ 0xf2, opt!(rex_prfx), 0x0f, 0xd0, rm ] = binary("addsubps",decode_rm,addsubps),

        // HADDPD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x7c, rm ] = binary("haddpd",decode_rm,haddpd),

        // HADDPS
        [ 0xf2, opt!(rex_prfx), 0x0f, 0x7c, rm ] = binary("haddps",decode_rm,haddps),

        // HSUBPD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x7d, rm ] = binary("hsubpd",decode_rm,hsubpd),

        // HSUBPS
        [ 0xf2, opt!(rex_prfx), 0x0f, 0x7d, rm ] = binary("hsubps",decode_rm,hsubps),

        // LDDQU
        [ 0xf2, opt!(rex_prfx), 0x0f, 0xf0, rm ] = binary("lddqu",decode_rm,lddqu),

        // MONITOR
        [ 0x0f, 0x01, 0xc8 ] = nonary("monitor",monitor),

        // MOVDDUP
        [ 0xf2, opt!(rex_prfx), 0x0f, 0x12, rm ] = binary("movddup",decode_rm,movddup),

        // MOVSHDUP
        [ 0xf3, opt!(rex_prfx), 0x0f, 0x16, rm ] = binary("movshdup",decode_rm,movshdup),

        // MUVSLDUP
        [ 0xf3, opt!(rex_prfx), 0x0f, 0x12, rm ] = binary("movsldup",decode_rm,movsldup),

        // MWAIT
        [ 0x0f, 0x01, 0xc9 ] = nonary("mwait",mwait),

        // PALIGNR
        [ opt!(rex_prfx), 0x0f, 0x3a, 0x0f, rm, imm8 ] = trinary("palignr",decode_rmi,palignr),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x3a, 0x0f, rm, imm8 ] = trinary("palignr",decode_rmi,palignr),

        // PABS*
        [       opt!(rex_prfx), 0x0f, 0x38, 0x1c, rm ] = binary("pabsb",decode_rm,pabsb),
        [       opt!(rex_prfx), 0x0f, 0x38, 0x1d, rm ] = binary("pabsw",decode_rm,pabsw),
        [       opt!(rex_prfx), 0x0f, 0x38, 0x1e, rm ] = binary("pabsd",decode_rm,pabsd),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x38, 0x1c, rm ] = binary("pabsb",decode_rm,pabsb),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x38, 0x1d, rm ] = binary("pabsw",decode_rm,pabsw),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x38, 0x1e, rm ] = binary("pabsd",decode_rm,pabsd))

        // PHADD*
}


pub fn sse4(rm: Rc<Disassembler<Amd64>>, imm8: Rc<Disassembler<Amd64>>,
            rex_prfx: Rc<Disassembler<Amd64>>, rexw_prfx: Rc<Disassembler<Amd64>>) -> Rc<Disassembler<Amd64>> {
    new_disassembler!(Amd64 =>
        // BLENDPD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x3a, 0x0c, rm, imm8 ] = trinary("blendpd",decode_rmi,blendpd),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x38, 0x15, rm ] = trinary("blendpd",decode_rm0,blendps),

        // BLENDPS
        [ 0x66, opt!(rex_prfx), 0x0f, 0x3a, 0x0d, rm, imm8 ] = trinary("blendps",decode_rmi,blendps),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x38, 0x14, rm ] = trinary("blendps",decode_rm0,blendps),

        // XXX: PBLENDVB

        // DPPD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x3a, 0x41, rm, imm8 ] = trinary("dppd",decode_rmi,dppd),

        // DPPS
        [ 0x66, opt!(rex_prfx), 0x0f, 0x3a, 0x40, rm, imm8 ] = trinary("dpps",decode_rmi,dpps),

        // EXTRACTPS
        [ 0x66, opt!(rex_prfx), 0x0f, 0x3a, 0x17, rm, imm8 ] = trinary("extractps",decode_rmi,extractps),

        // INSERTPS
        [ 0x66, opt!(rex_prfx), 0x0f, 0x3a, 0x21, rm, imm8 ] = trinary("insertps",decode_rmi,insertps),

        // MPSADBW
        [ 0x66, opt!(rex_prfx), 0x0f, 0x3a, 0x42, rm, imm8 ] = trinary("mpsadbw",decode_rmi,mpsadbw),

        // XXX: MOVNTDQA

        // PBLENDW
        [ 0x66, opt!(rex_prfx), 0x0f, 0x3a, 0x0e, rm, imm8 ] = trinary("pblendbw",decode_rmi,pblendbw),

        // PCMPESTRI
        [ 0x66, opt!(rex_prfx), 0x0f, 0x3a, 0x61, rm, imm8 ] = trinary("pcmpestri",decode_rmi,pcmpestri),

        // PCMPESTRM
        [ 0x66, opt!(rex_prfx), 0x0f, 0x3a, 0x60, rm, imm8 ] = trinary("pcmpestrm",decode_rmi,pcmpestrm),

        // PCMPISTRI
        [ 0x66, opt!(rex_prfx), 0x0f, 0x3a, 0x63, rm, imm8 ] = trinary("pcmpistri",decode_rmi,pcmpistri),

        // PCMPISTRM
        [ 0x66, opt!(rex_prfx), 0x0f, 0x3a, 0x62, rm, imm8 ] = trinary("pcmpistrm",decode_rmi,pcmpistrm),

        // PEXTRB
        [ 0x66, opt!(rex_prfx), 0x0f, 0x3a, 0x14, rm, imm8 ] = trinary("pextrb",decode_mri,pextrb),

        // PEXTRD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x3a, 0x16, rm, imm8 ] = trinary("extrd",decode_mri,pextrd),

        // PEXTRQ
        [ 0x66, opt!(rexw_prfx), 0x0f, 0x3a, 0x16, rm, imm8 ] = trinary("extrq",decode_mri,pextrq),

        // PEXTRW
        [ 0x66, opt!(rex_prfx), 0x0f, 0x3a, 0x15, rm, imm8 ] = trinary("extrw",decode_mri,pextrw),

        // PINSRB
        [ 0x66, opt!(rex_prfx), 0x0f, 0x3a, 0x20, rm, imm8 ] = trinary("pinsrb",decode_rmi,pinsrb),

        // PINSRD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x3a, 0x22, rm, imm8 ] = trinary("pinsrd",decode_rmi,pinsrd),

        // PINSRQ
        [ 0x66, opt!(rexw_prfx), 0x0f, 0x3a, 0x22, rm, imm8 ] = trinary("pinsrq",decode_rmi,pinsrq),

        // ROUNDPD
        [ 0x66, opt!(rexw_prfx), 0x0f, 0x3a, 0x09, rm, imm8 ] = trinary("roundpd",decode_rmi,roundpd),

        // ROUNDPS
        [ 0x66, opt!(rexw_prfx), 0x0f, 0x3a, 0x08, rm, imm8 ] = trinary("roundpd",decode_rmi,roundps),

        // ROUNDSD
        [ 0x66, opt!(rexw_prfx), 0x0f, 0x3a, 0x0b, rm, imm8 ] = trinary("roundpd",decode_rmi,roundsd),

        // ROUNDSS
        [ 0x66, opt!(rexw_prfx), 0x0f, 0x3a, 0x0a, rm, imm8 ] = trinary("roundpd",decode_rmi,roundss))
}

pub fn avx(vex_prfx: Rc<Disassembler<Amd64>>, rm: Rc<Disassembler<Amd64>>) -> Rc<Disassembler<Amd64>> {
    new_disassembler!(Amd64 =>
        // VADD*
        [ vex_prfx, 0x66, 0x0f, 0x58, rm ] = trinary("vaddpd",decode_rvm,vaddpd),
        [ vex_prfx, 0x0f, 0x58, rm ] = trinary("vaddps",decode_rvm,vaddps),
        [ vex_prfx, 0xf2, 0x0f, 0x58, rm ] = trinary("vaddsd",decode_rvm,vaddsd),
        [ vex_prfx, 0xf3, 0x0f, 0x58, rm ] = trinary("vaddss",decode_rvm,vaddss))
        /*

        // VADDSUBP*
        [ vec_prfx, 0x66, 0x0f, 0xd0, rm ] = trinary("vaddsubpd",decode_rvm,vaddsubpd),
        [ vec_prfx, 0xf2, 0x0f, 0xd0, rm ] = trinary("vaddsubps",decode_rvm,vaddsubps),

        // VAES*
        [ vec_prfx, 0x66, 0x0f, 0x38, 0xde, rm ] = trinary("vaesdec",decode_rvm,vaesdec),
        [ vec_prfx, 0x66, 0x0f, 0x38, 0xdf, rm ] = trinary("vaesdeclast",decode_rvm,vaesdeclast),
        [ vec_prfx, 0x66, 0x0f, 0x38, 0xdc, rm ] = trinary("vaesenc",decode_rvm,vaesenc),
        [ vec_prfx, 0x66, 0x0f, 0x38, 0xdd, rm ] = trinary("vaesenclast",decode_rvm,vaesenclast),
        [ vec_prfx, 0x66, 0x0f, 0x38, 0xdb, rm ] = trinary("vaesimc",decode_rvm,vaesimc),
        [ vec_prfx, 0x66, 0x0f, 0x3a, 0xdf, rm ] = trinary("vaeskeygenassist",decode_rvm,vaeskeygenassist),

        // VANDP*
        [ vec_prfx, 0x66, 0x0f, 0x54, rm ] = trinary("vandpd",decode_rvm,vandpd),
        [ vec_prfx, 0x0f, 0x54, rm ] = trinary("vandps",decode_rvm,vandps),

        // VANDNP*
        [ vec_prfx, 0x66, 0x0f, 0x55, rm ] = trinary("vandnpd",decode_rvm,vandnpd),
        [ vec_prfx, 0x0f, 0x55, rm ] = trinary("vandnps",decode_rvm,vandnps),

        // VBLENDPD
        [ vec_prfx, 0x66, 0x0f, 0x3a, 0x0d, rm, imm8 ] = quintary("vblendpd",decode_rvmi,vblendpd),
        [ vec_prfx, 0x0f, 0x3a, 0x0d, rm, imm8 ] = quintary("vblendpd",decode_rvmi,vblendpd),

        // BEXTR
        [ vec_prfx, 0x0f, 0x38, 0xf7, rm ] = trinary("bextr",decode_rvm,vandps),

        // VBLENDPS
        [ vec_prfx, 0x66, 0x0f, 0x3a, 0x0c, rm, imm8 ] = quintary("vblendps",decode_rvmi,vblendps),
        [ vec_prfx, 0x0f, 0x3a, 0x0c, rm, imm8 ] = quintary("vblendps",decode_rvmi,vblendps),

        // VBLENDVP*
        [ vec_prfx, 0x66, 0x0f, 0x3a, 0x4b, rm, is4 ] = quintary("vblendvpd",decode_rvmr,vblendvpd),
        [ vec_prfx, 0x66, 0x0f, 0x3a, 0x4a, rm, is4 ] = quintary("vblendvps",decode_rvmr,vblendvps),

        // VCMPP*
        [ vec_prfx, 0x66, 0x0f, 0xc2, rm, imm8 ] = quintary("vcmppd",decode_rvmi,vcmppd),
        [ vec_prfx, 0x0f, 0xc2, rm, imm8 ] = quintary("vcmpps",decode_rvmi,vcmpps),

        // VCMPS*
        [ vec_prfx, 0xf2, 0x0f, 0xc2, rm, imm8 ] = quintary("vcmpsd",decode_rvmi,vcmpsd),
        [ vec_prfx, 0xf3, 0x0f, 0xc2, rm, imm8 ] = quintary("vcmpss",decode_rvmi,vcmpss),

        // VCOMIS*
        [ vec_prfx, 0x66, 0x0f, 0x2f, rm ] = binary("vcomisd",decode_rm,vcomisd),
        [ vec_prfx, 0x0f, 0x2f, rm ] = binary("vcomiss",decode_rm,vcomiss),

        // VCVTDQ2PD
        [ vex_prfx, 0xf3, 0x0f, 0xe6, rm ] = binary("vcvtdq2pd",decode_rm,vcvtdq2pd),

        // VCVTDQ2PS
        [ vex_prfx, 0x0f, 0x5b, rm ] = binary("vcvtdq2ps",decode_rm,vcvtdq2ps),

        // VCVTPD2DQ
        [ vex_prfx, 0xf2, 0x0f, 0xe6, rm ] = binary("vcvtdq2pd",decode_rm,vcvtpd2dq),

        // VCVTPD2PS
        [ vex_prfx, 0x66, 0x0f, 0x5a, rm ] = binary("vcvtpd2ps",decode_rm,vcvtpd2ps),

        // VCVTPS2DQ
        [ vex_prfx, 0x66, 0x0f, 0x5b, rm ] = binary("vcvtps2dq",decode_rm,vcvtps2dq),

        // VCVTPS2PD
        [ vex_prfx, 0x0f, 0x5a, rm ] = binary("vcvtps2pd",decode_rm,vcvtps2pd),

        // VCVTSD2SI
        [ vex_prfx, 0xf2, 0x0f, 0x2d, rm ] = binary("vcvtsd2si",decode_rm,vcvtsd2si),

        // VCVTSD2SS
        [ vex_prfx, 0xf2, 0x0f, 0x5a, rm ] = binary("vcvtsd2ss",decode_rm,vcvtsd2ss),

        // VCVTSI2SD
        [ vex_prfx, 0xf2, 0x0f, 0x2a, rm ] = binary("vcvtsi2sd",decode_rm,vcvtsi2sd),

        // VCVTSS2SD
        [ vex_prfx, 0xf3, 0x0f, 0x5a, rm ] = binary("vcvtss2sd",decode_rm,vcvtss2sd),

        //si2sd
        //si2ss
        //ss2sd
        //ss2si

        // VCVTTPD2DQ
        [ vex_prfx, 0x66, 0x0f, 0xe6, rm ] = binary("vcvttpd2dq",decode_rm,vcvttpd2dq),

        // VCVTTPS2DQ
        [ vex_prfx, 0xf3, 0x0f, 0x5b, rm ] = binary("vcvttps2dq",decode_rm,vcvttps2dq),

        // VCVTTSD2SI
        [ vex_prfx, 0xf2, 0x0f, 0x2c, rm ] = binary("vcvttsd2si",decode_rm,vcvttsd2si),

        //tss2si

        // VDIV*
        [ vex_prfx, 0x0f, 0x5e, rm ] = trinary("vdivps",decode_rmv,vdivps),
        [ vex_prfx, 0x66, 0x0f, 0x5e, rm ] = trinary("vdivpd",decode_rmv,vdivpd),
        [ vex_prfx, 0xf3, 0x0f, 0x5e, rm ] = trinary("vdivss",decode_rmv,vdivss),
        [ vex_prfx, 0xf2, 0x0f, 0x5e, rm ] = trinary("vdivsd",decode_rmv,vdivsd),

        // VDPPD
        [ vex_prfx, 0x66, 0x0f, 0x3a, 0x41, rm, imm8 ] = quintary("dppd",decode_vrmi,vdppd),

        // VDPPS
        [ vex_prfx, 0x66, 0x0f, 0x3a, 0x40, rm, imm8 ] = quintary("dpps",decode_vrmi,vdpps),

        // VEXTRACTPS
        [ vex_prfx, 0x66, 0x0f, 0x3a, 0x17, rm, imm8 ] = quintary("extractps",decode_vrmi,vextractps),

        // VHADDPD
        [ vex_prfx, 0x66, 0x0f, 0x7c, rm ] = trinary("vhaddpd",decode_rvm,vhaddpd),

        // VHADDPS
        [ vex_prfx, 0xf2, 0x0f, 0x7c, rm ] = trinary("vhaddps",decode_rvm,vhaddps),

        // VHSUBPD
        [ vex_prfx, 0x66, 0x0f, 0x7d, rm ] = trinary("vhsubpd",decode_rvm,vhsubpd),

        // VHSUBPS
        [ vex_prfx, 0xf2, 0x0f, 0x7d, rm ] = trinary("vhsubps",decode_rvm,vhsubps),

        // VINSERTPS
        [ vex_prfx, 0x66, 0x0f, 0x3a, 0x21, rm, imm8 ] = quintary("insertps",decode_vrmi,vinsertps),

        // VLDDQU
        [ vex_prefx, 0xf2, 0x0f, 0xf0, rm ] = binary("vlddqu",decode_rm,vlddqu),

        // VLDMXCSR
        [ vex_prefx, 0x0f, 0xae, rm2 ] = unary("vldmxcsr",decode_m,ldmxcsr),

        // MASKMOVDQU
        [ vex_prfx, 0x66, 0x0f, 0xf7, rm ] = binary("maskmovdqu",decode_rm,maskmovdqu),

        // VMAX*
        [ vex_prfx, 0x66, 0x0f, 0x5f, rm ] = trinary("vmaxpd",decode_rvm,vmaxpd),
        [ vex_prfx, 0xf2, 0x0f, 0x5f, rm ] = trinary("vmaxsd",decode_rvm,vmaxsd),
        [ vex_prfx, 0x0f, 0x5f, rm       ] = trinary("vmaxps",decode_rvm,vmaxps),
        [ vex_prfx, 0xf3, 0x0f, 0x5f, rm ] = trinary("vmaxss",decode_rvm,vmaxss),

        // VMIN*
        [ vex_prfx, 0x66, 0x0f, 0x5d, rm ] = trinary("vminpd",decode_rvm,vminpd),
        [ vex_prfx, 0xf2, 0x0f, 0x5d, rm ] = trinary("vminsd",decode_rvm,vminsd),
        [ vex_prfx, 0x0f, 0x5d, rm       ] = trinary("vminps",decode_rvm,vminps),
        [ vex_prfx, 0xf3, 0x0f, 0x5d, rm ] = trinary("vminss",decode_rvm,vminss),

        // MOVAPD
        [ vex_prfx, 0x66, 0x0f, 0x28, rm ] = binary("movapd",decode_rm,movapd),
        [ vex_prfx, 0x66, 0x0f, 0x29, rm ] = binary("movapd",decode_mr,movapd),

        // MOVAPS
        [ vex_prfx, 0x0f, 0x28, rm ] = binary("movaps",decode_rm,movaps),
        [ vex_prfx, 0x0f, 0x29, rm ] = binary("movaps",decode_mr,movaps),

        // MOVD
        [ vex_prfx, 0x66, 0x0f, 0x6e, rm ] = binary("movd",decode_rm,movd),
        [ vex_prfx, 0x66, 0x0f, 0x7e, rm ] = binary("movd",decode_mr,movd),

        // MOVDDUP
        [ vex_prfx, 0xf2, 0x0f, 0x12, rm ] = binary("movddup",decode_rm,movddup),

        // MOVDQA
        [ vex_prfx, 0x66, 0x0f, 0x6f, rm ] = binary("movdqa",decode_rm,movdaq),
        [ vex_prfx, 0x66, 0x0f, 0x7f, rm ] = binary("movdqa",decode_mr,movdqa),

        // MOVDQU
        [ vex_prfx, 0xf3, 0x0f, 0x6f, rm ] = binary("movdqu",decode_rm,movdqu),
        [ vex_prfx, 0xf3, 0x0f, 0x7f, rm ] = binary("movdqu",decode_mr,movdqu),

        // MOVHPS
        [ vex_prfx, 0x0f, 0x16, rm ] = binary("minhps",decode_rm,minhps),
        [ vex_prfx, 0x0f, 0x17, rm ] = binary("minhps",decode_mr,minhps),

        // VMOVHPD
        [ vex_prfx, 0x66, 0x0f, 0x16, rm ] = trinary("vmovhpd",decode_rvm,vmovhpd),
        [ vex_prfx, 0x66, 0x0f, 0x17, rm ] = binary("vmovhpd",decode_mr,movhpd),

        // VMOVHPS
        [ vex_prfx, 0x0f, 0x16, rm ] = trinary("vmovhps",decode_rvm,vmovhps),
        [ vex_prfx, 0x0f, 0x17, rm ] = binary("vmovhps",decode_mr,movhps),

        // VMOVLPD
        [ vex_prfx, 0x66, 0x0f, 0x12, rm ] = trinary("vmovlpd",decode_rvm,vmovlpd),
        [ vex_prfx, 0x66, 0x0f, 0x13, rm ] = binary("movlpd",decode_mr,movlpd),

        // VMOVLPS
        [ vex_prfx, 0x0f, 0x12, rm ] = trinary("vmovlps",decode_rvm,vmovlps),
        [ vex_prfx, 0x0f, 0x13, rm ] = binary("movlps",decode_mr,movlps),

        // MOVMSKPD
        [ vex_prfx, 0x66, 0x0f, 0x50, rm ] = binary("movmskpd",decode_rm,movmskpd),

        // MOVMSKPS
        [ vex_prfx, 0x0f, 0x50, rm ] = binary("movmskps",decode_rm,movmskps),

        // MOVNTQA
        [ vex_prfx, 0x66, 0x0f, 0x38, 0x2a, rm ] = binary("movntqa",decode_rm,movntdqa),

        // MOVNTDQ
        [ vex_prfx, 0x0f, 0xe7, rm ] = binary("movntq",decode_mr,movntq),

        // MOVNTP*
        [ vex_prfx, 0x66, 0x0f, 0x2b, rm ] = binary("movntpd",decode_mr,movntpd),
        [ vex_prfx, 0x0f, 0x2b, rm ] = binary("movntps",decode_mr,movntps),

        // MOVDQ2Q
        [ vex_prfx, 0xf2, 0x0f, 0xd6, rm ] = binary("movdq2q",decode_rm,movdq2q),

        // MOVHPD
        [ vex_prfx, 0x66, 0x0f, 0x16, rm ] = binary("movhpd",decode_rm,movhpd),
        [ vex_prfx, 0x66, 0x0f, 0x17, rm ] = binary("movhpd",decode_mr,movhpd),

        // MOVQ
        [ vex_prfx, 0xf3, 0x0f, 0x7e, rm ] = binary("movq",decode_rm,movq),
        [ vex_prfx, 0x66, 0x0f, 0xd6, rm ] = binary("movq",decode_mr,movq),

        // VMOVSD
        [ vex_prfx, 0xf2, 0x0f, 0x10, rm ] = binary("vmovsd",decode_rvm,vmovsd),
        [ vex_prfx, 0xf2, 0x0f, 0x11, rm ] = binary("vmovsd",decode_mvr,vmovsd),

        // MOVSHDUP
        [ vex_prfx, 0xf3, 0x0f, 0x16, rm ] = binary("movshdup",decode_rm,movshdup),

        // MUVSLDUP
        [ vex_prfx, 0xf3, 0x0f, 0x12, rm ] = binary("movsldup",decode_rm,movsldup),

        // VMOVSS
        [ vex_prfx, 0xf3, 0x0f, 0x10, rm ] = binary("vmovss",decode_rvm,vmovss),
        [ vex_prfx, 0xf3, 0x0f, 0x11, rm ] = binary("vmovss",decode_mvr,vmovss),

        // MOVUPD
        [ vex_prfx, 0x66, 0x0f, 0x10, rm ] = binary("movupd",decode_rm,movupd),
        [ vex_prfx, 0x66, 0x0f, 0x11, rm ] = binary("movupd",decode_mr,movupd),

        // MOVUPS
        [ vex_prfx, 0x0f, 0x10, rm ] = binary("movups",decode_rm,movups),
        [ vex_prfx, 0x0f, 0x11, rm ] = binary("movups",decode_mr,movups),

        // MPSADBW
        [ vex_prfx, 0x66, 0x0f, 0x3a, 0x42, rm, imm8 ] = quintary("mpsadbw",decode_vrmi,vmpsadbw),

        // MUL*
        [ vex_prfx, 0x0f, 0x59, rm       ] = trinary("mulps",decode_rvm,mulps),
        [ vex_prfx, 0xf3, 0x0f, 0x59, rm ] = trinary("mulss",decode_rvm,mulss),
        [ vex_prfx, 0x66, 0x0f, 0x59, rm ] = trinary("mulpd",decode_rvm,mulpd),
        [ vex_prfx, 0xf2, 0x0f, 0x59, rm ] = trinary("mulsd",decode_rvm,mulsd),

        // VORPD
        [ vex_prfx, 0x66, 0x0f, 0x56, rm ] = binary("vorpd",decode_rvm,vorpd),

        // VORPS
        [ vex_prfx, 0x0f, 0x56, rm ] = binary("vorps",decode_rvm,vorps),

        // VPABS*
        [ vex_prfx, 0x66, 0x0f, 0x38, 0x1c, rm ] = binary("vpabsb",decode_rm,vpabsb),
        [ vex_prfx, 0x66, 0x0f, 0x38, 0x1d, rm ] = binary("vpabsw",decode_rm,vpabsw),
        [ vex_prfx, 0x66, 0x0f, 0x38, 0x1e, rm ] = binary("vpabsd",decode_rm,vpabsd),

        // VPACKSS*
        [ vex_prfx, 0x66, 0x0f, 0x63, rm ] = trinary("vpacksswb",decode_rvm,vpacksswb),
        [ vex_prfx, 0x66, 0x0f, 0x6b, rm ] = trinary("vpackssdw",decode_rvm,vpackssdw),

        // VPACKUS*
        [ vex_prfx, 0x66, 0x0f, 0x38, 0x2b, rm ] = trinary("vpackusdw",decode_rvm,vpackusdw),
        [ vex_prfx, 0x66, 0x0f, 0x67, rm ] = trinary("vpackuswb",decode_rvm,vpackuswb),

        // VPADD*
        [ vex_prfx, 0x66, 0x0f, 0xfc, rm ] = trinary("vpaddb",decode_rvm,vpaddb),
        [ vex_prfx, 0x66, 0x0f, 0xfd, rm ] = trinary("vpaddw",decode_rvm,vpaddw),
        [ vex_prfx, 0x66, 0x0f, 0xfe, rm ] = trinary("vpaddd",decode_rvm,vpaddd),
        [ vex_prfx, 0x66, 0x0f, 0xd4, rm ] = trinary("vpaddq",decode_rvm,vpaddq),

        // VPADDS*
        [ vex_prfx, 0x66, 0x0f, 0xec, rm ] = binary("vpaddsb",decode_rvm,vpaddsb),
        [ vex_prfx, 0x66, 0x0f, 0xed, rm ] = binary("vpaddsw",decode_rvm,vpaddsw),

        // VPADDUS*
        [ vex_prfx, 0x66, 0x0f, 0xdc, rm ] = trinary("vpaddusb",decode_rvm,vpaddusb),
        [ vex_prfx, 0x66, 0x0f, 0xdd, rm ] = trinary("vpaddusw",decode_rvm,vpaddusw),

        // VPALIGNR
        [ vex_prfx, 0x66, 0x0f, 0x3a, 0x0f, rm, imm8 ] = quinary("vpalignr",decode_rvmi,vpalignr),

        // VPAND
        [ vex_prfx, 0x66, 0x0f, 0xdb, rm ] = trinary("vpand",decode_rvm,vpand),

        // VPANDN
        [ vex_prfx, 0x66, 0x0f, 0xdf, rm ] = trinary("vpandn",decode_rvm,vpandn),

        // VPAVG*
        [ vex_prfx, 0x0f, 0xe0, rm ] = trinary("vpavgb",decode_rvm,vpavgb),
        [ vex_prfx, 0x0f, 0xe3, rm ] = trinary("vpavgw",decode_rvm,vpavgw),

        // VPBLENDVB
        [ vex_prfx, 0x66, 0x0f, 0x3a, 0x4c, rm, is4 ] = quinary("vpblendvb",decode_rvmr,vpblendvb),

        // VPBLENDW
        [ vex_prfx, 0x66, 0x0f, 0x3a, 0x0e, rm, imm8 ] = quinary("vpblendw",decode_rvmi,vpblendw),

        // VPCLMULQDQ
        [ vex_prfx, 0x66, 0x0f, 0x3a, 0x44, rm, imm8 ] = quinary("vpclmulqdq",decode_rvmi,vpclmulqdq),

        // VPCMPEQ*
        [ vex_prfx, 0x66, 0x0f, 0x74, rm ] = trinary("vpcmpeqb",decode_rvm,vpcmpeqb),
        [ vex_prfx, 0x66, 0x0f, 0x75, rm ] = trinary("vpcmpeqw",decode_rvm,vpcmpeqw),
        [ vex_prfx, 0x66, 0x0f, 0x76, rm ] = trinary("vpcmpeqd",decode_rvm,vpcmpeqd),
        [ vex_prfx, 0x66, 0x0f, 0x38, 0x29, rm ] = trinary("vpcmpeqd",decode_rvm,vpcmpeqq),

        // PCMPESTRI
        [ vex_prfx, 0x66, 0x0f, 0x3a, 0x61, rm, imm8 ] = trinary("pcmpestri",decode_rmi,pcmpestri),

        // PCMPESTRM
        [ vex_prfx, 0x66, 0x0f, 0x3a, 0x60, rm, imm8 ] = trinary("pcmpestrm",decode_rmi,pcmpestrm),

        // VPCMPGT*
        [ vex_prfx, 0x66, 0x0f, 0x64, rm ] = trinary("vpcmpgtb",decode_rvm,vpcmpgtb),
        [ vex_prfx, 0x66, 0x0f, 0x65, rm ] = trinary("vpcmpgtw",decode_rvm,vpcmpgtw),
        [ vex_prfx, 0x66, 0x0f, 0x66, rm ] = trinary("vpcmpgtd",decode_rvm,vpcmpgtd),
        [ vex_prfx, 0x66, 0x0f, 0x38, 0x37, rm ] = trinary("vpcmpgtq",decode_rvm,vpcmpgtq),

        // PCMPISTRI
        [ vex_prfx, 0x66, 0x0f, 0x3a, 0x63, rm, imm8 ] = trinary("pcmpistri",decode_rmi,pcmpistri),

        // PCMPISTRM
        [ vex_prfx, 0x66, 0x0f, 0x3a, 0x62, rm, imm8 ] = trinary("pcmpistrm",decode_rmi,pcmpistrm),

        // PEXT*
        [ vex_prfx, 0x66, 0x0f, 0x3a, 0x14, rm, imm8 ] = trinary("pextrb",decode_mri,pextrb),
        [ vex_prfx, 0x66, 0x0f, 0x3a, 0x16, rm, imm8 ] = trinary("pextrd",decode_mri,pextrd),
        [ vex_prfx, 0x66, 0x0f, 0x3a, 0x15, rm, imm8 ] = trinary("pextrw",decode_mri,pextrw),
        [ vex_prfx, 0x66, 0x0f, 0xc5, rm, imm8 ] = trinary("pextrw",decode_rmi,pextrw)
)*/
        // XXX WIP XXX

        /*
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

        [ 0x0f, 0x0d, rm1 ] = unary("prefetchw",decode_m,prefetchw),
        [ 0x0f, 0x0d, rm2 ] = unary("prefetchwt1",decode_m,prefetchwt1),

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

        // PAUSE
        [ 0xf3, 0x90 ] = nonary("pause",pause),

        // PMADDWD
        [ 0x66, opt!(rex_prfx), 0x0f, 0xf5, rm ] = binary("pmaddwd",decode_rm,pmadwd),

        // PMUL*W
        [ 0x66, opt!(rex_prfx), 0x0f, 0xe5, rm ] = binary("pmulhw",decode_rm,pmulhw),
        [ 0x66, opt!(rex_prfx), 0x0f, 0xd5, rm ] = binary("pmullw",decode_rm,pmullw),

        // PMULUDQ
        [ opt!(0x66), opt!(rex_prfx), 0x0f, 0xf4, rm ] = binary("pcmpgtd",decode_rm,pmuludq),

        // POR
        [ 0x66, opt!(rex_prfx), 0x0f, 0xeb, rm ] = binary("pcmpgtd",decode_rm,por),

        // PSHUFD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x70, rm, imm8 ] = trinary("pshufd",decode_rmi,pshufd),

        // PSHUFHW
        [ 0xf3, opt!(rex_prfx), 0x0f, 0x70, rm, imm8 ] = trinary("pshufhw",decode_rmi,pshufhw),
        [ 0xf2, opt!(rex_prfx), 0x0f, 0x70, rm, imm8 ] = trinary("pshuflw",decode_rmi,pshuflw),

        // PSLLD
        [ 0x66, opt!(rex_prfx), 0x0f, 0xf1, rm        ] = binary("psllw",decode_rm,psllw),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x71, rm6, imm8 ] = binary("psllw",decode_mi,psllw),
        [ 0x66, opt!(rex_prfx), 0x0f, 0xf2, rm        ] = binary("pslld",decode_rm,pslld),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x72, rm6, imm8 ] = binary("pslld",decode_mi,pslld),
        [ 0x66, opt!(rex_prfx), 0x0f, 0xf3, rm        ] = binary("psllq",decode_rm,psllq),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x73, rm6, imm8 ] = binary("psllq",decode_mi,psllq),

        // PSLLDQ
        [ 0x66, opt!(rex_prfx), 0x0f, 0x73, rm7, imm8 ] = binary("pslldq",decode_mi,pslldq),

        // PSRAD
        [ 0x66, opt!(rex_prfx), 0x0f, 0xe2, rm        ] = binary("psrad",decode_rm,psrad),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x72, rm4, imm8 ] = binary("psrad",decode_mi,psrad),

        // PSRAW
        [ 0x66, opt!(rex_prfx), 0x0f, 0xe1, rm        ] = binary("psraw",decode_rm,psarw),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x71, rm4, imm8 ] = binary("psraw",decode_mi,psarw),

        // PSRLW
        [ 0x66, opt!(rex_prfx), 0x0f, 0xd1, rm        ] = binary("psrlw",decode_rm,psrlw),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x71, rm2, imm8 ] = binary("psrlw",decode_mi,psrlw),
        [ 0x66, opt!(rex_prfx), 0x0f, 0xd2, rm        ] = binary("psrld",decode_rm,psrld),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x72, rm2, imm8 ] = binary("psrld",decode_mi,psrld),
        [ 0x66, opt!(rex_prfx), 0x0f, 0xd3, rm        ] = binary("psrlq",decode_rm,psrlq),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x73, rm2, imm8 ] = binary("psrlq",decode_mi,psrlq),

        // PSRLDQ
        [ 0x66, opt!(rex_prfx), 0x0f, 0x73, rm3, imm8 ] = binary("psrldq",decode_mi,psrldq),

        // PSUBB
        [ 0x66, opt!(rex_prfx), 0x0f, 0xf8, rm ] = binary("psubb",decode_rm,psubb),
        [ 0x66, opt!(rex_prfx), 0x0f, 0xf9, rm ] = binary("psubw",decode_rm,psubw),
        [ 0x66, opt!(rex_prfx), 0x0f, 0xfa, rm ] = binary("psubd",decode_rm,psubd),
        [ opt!(0x66), opt!(rex_prfx), 0x0f, 0xfb, rm ] = binary("psubq",decode_rm,psubq),

        // PSUBS*
        [ 0x66, opt!(rex_prfx), 0x0f, 0xe8, rm ] = binary("psubsb",decode_rm,psubsb),
        [ 0x66, opt!(rex_prfx), 0x0f, 0xe9, rm ] = binary("psubsw",decode_rm,pusbsw),

        // PSUBUS*
        [ 0x66, opt!(rex_prfx), 0x0f, 0xd8, rm ] = binary("psubusb",decode_rm,psubusb),
        [ 0x66, opt!(rex_prfx), 0x0f, 0xd9, rm ] = binary("psubusw",decode_rm,psubusw),

        // PMOVMSKB
        [ 0x66, opt!(rex_prfx), opt!(rex_prfx), 0x0f, 0xd7, rm ] = binary("pmovmskb",decode_rm,pmovmskb),

        // PMAX*
        [ 0x66, opt!(rex_prfx), 0x0f, 0xee, rm ] = binary("pmaxsw",decode_rm,pmaxsw),
        [ 0x66, opt!(rex_prfx), 0x0f, 0xde, rm ] = binary("pmaxub",decode_rm,pmaxub),

        // PMIN*
        [ 0x66, opt!(rex_prfx), 0x0f, 0xea, rm ] = binary("pminsw",decode_rm,pminsw),
        [ 0x66, opt!(rex_prfx), 0x0f, 0xda, rm ] = binary("pminub",decode_rm,pminub),

        // PUNPCKHBW
        [ 0x66, opt!(rex_prfx), 0x0f, 0x68, rm ] = binary("punpckhbw",decode_rm,punpckhbw),

        // PUNPCKHWD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x69, rm ] = binary("punpckhwd",decode_rm,punckhwd),

        // PUNPCKHDQ
        [ 0x66, opt!(rex_prfx), 0x0f, 0x6a, rm ] = binary("punpckhwd",decode_rm,punpckhdq),

        // PUNPCKHQDQ
        [ 0x66, opt!(rex_prfx), 0x0f, 0x6d, rm ] = binary("punpckhqdq",decode_rm,punpckhqdq),

        // PUNPCKLBW
        [ 0x66, opt!(rex_prfx), 0x0f, 0x60, rm ] = binary("punpcklbw",decode_rm,punpcklbw),

        // PUNPCKLDQ
        [ 0x66, opt!(rex_prfx), 0x0f, 0x61, rm ] = binary("punpckldq",decode_rm,punpckldq),

        // PUNPCKLQDQ
        [ 0x66, opt!(rex_prfx), 0x0f, 0x62, rm ] = binary("punpcklqdq",decode_rm,puncklqdq),

        // PUNPCKLWD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x6c, rm ] = binary("punpcklwd",decode_rm,puncklwd),

        // PXOR
        [ 0x66, opt!(rex_prfx), 0x0f, 0xef, rm ] = binary("pxor",decode_rm,pxor),

        // SHUFPD
        [ 0x66, opt!(rex_prfx), 0x0f, 0xc6, rm, imm8 ] = trinary("shufpd",decode_rmi,shufpd),

        // SQRTPD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x51, rm ] = binary("sqrtpd",decode_rm,sqrtpd),

        // SQRTSD
        [ 0xf2, opt!(rex_prfx), 0x0f, 0x51, rm ] = binary("sqrtsd",decode_rm,sqrtsd),

        // SUBPD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x5c, rm ] = binary("subpd",decode_rm,subpd),

        // SUBSD
        [ 0xf2, opt!(rex_prfx), 0x0f, 0x5c, rm ] = binary("subsd",decode_rm,subsd),

        // UCOMISD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x2e, rm ] = binary("ucomisd",decode_rm,ucomisd),

        // UNPCKHPD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x15, rm ] = binary("unpckhpd",decode_rm,unpckhpd),

        // UNPCKLPD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x14, rm ] = binary("unpcklpd",decode_rm,unpcklpd),

        // XORPD
        [ 0x66, opt!(rex_prfx), 0x0f, 0x57, rm ] = binary("xorpd",decode_rm,xorpd),

        // MOVD/MOVQ
        [ 0x66, opt!(rex_prfx), 0x0f, 0x6e, rm ] = binary("movd",decode_rm,mov),
        [ 0x66, opt!(rex_prfx), 0x0f, 0x7e, rm ] = binary("movd",decode_mr,mov),
        )*/
}

fn mpx() {}

fn bmi1() {/*
    new_disassembler!(Amd64 =>
        // BLSI
        [ vec_prfx, 0x66, 0x0f, 0x38, 0xf3, rm3 ] = binary("blsi",decode_vm,vblsi),

        // BLSMSK
        [ vec_prfx, 0x66, 0x0f, 0x38, 0xf3, rm2 ] = binary("blsmsk",decode_vm,blsmsk),

        // BLSR
        [ vec_prfx, 0x66, 0x0f, 0x38, 0xf3, rm1 ] = binary("blsr",decode_vm,blsr))*/
}

fn bmi2() {
}
