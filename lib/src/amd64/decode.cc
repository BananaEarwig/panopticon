/*
 * This file is part of Panopticon (http://panopticon.re).
 * Copyright (C) 2014 Panopticon authors
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

#include <functional>
#include <list>
#include <string>
#include <iomanip>

#include <panopticon/amd64/amd64.hh>
#include <panopticon/amd64/decode.hh>
#include <panopticon/amd64/traits.hh>

#include <panopticon/code_generator.hh>

using namespace po;
using namespace po::amd64;

memory po::amd64::byte(rvalue o) { return memory(o,1,LittleEndian,"ram"); }
memory po::amd64::word(rvalue o) { return memory(o,2,LittleEndian,"ram"); }
memory po::amd64::dword(rvalue o) { return memory(o,4,LittleEndian,"ram"); }
memory po::amd64::qword(rvalue o) { return memory(o,8,LittleEndian,"ram"); }

memory po::amd64::byte(uint64_t o) { return byte(constant(o)); }
memory po::amd64::word(uint64_t o) { return word(constant(o)); }
memory po::amd64::dword(uint64_t o) { return dword(constant(o)); }
memory po::amd64::qword(uint64_t o) { return qword(constant(o)); }

variable po::amd64::decode_reg8(unsigned int r_reg,bool rex)
{
	switch(r_reg)
	{
		case 0: return to_variable(al);
		case 1: return to_variable(cl);
		case 2: return to_variable(dl);
		case 3: return to_variable(bl);
		case 4: return rex ? to_variable(spl) : to_variable(ah);
		case 5: return rex ? to_variable(bpl) : to_variable(ch);
		case 6: return rex ? to_variable(sil) : to_variable(dh);
		case 7: return rex ? to_variable(dil) : to_variable(bh);
		case 8: return to_variable(r8l);
		case 9: return to_variable(r9l);
		case 10: return to_variable(r10l);
		case 11: return to_variable(r11l);
		case 12: return to_variable(r12l);
		case 13: return to_variable(r13l);
		case 14: return to_variable(r14l);
		case 15: return to_variable(r15l);
		default: ensure(false);
	}
}

variable po::amd64::decode_reg16(unsigned int r_reg)
{
	switch(r_reg)
	{
		case 0: return to_variable(ax);
		case 1: return to_variable(cx);
		case 2: return to_variable(dx);
		case 3: return to_variable(bx);
		case 4: return to_variable(sp);
		case 5: return to_variable(bp);
		case 6: return to_variable(si);
		case 7: return to_variable(di);
		case 8: return to_variable(r8w);
		case 9: return to_variable(r9w);
		case 10: return to_variable(r10w);
		case 11: return to_variable(r11w);
		case 12: return to_variable(r12w);
		case 13: return to_variable(r13w);
		case 14: return to_variable(r14w);
		case 15: return to_variable(r15w);
		default: ensure(false);
	}
}

variable po::amd64::decode_reg32(unsigned int r_reg)
{
	switch(r_reg)
	{
	case 0: return to_variable(eax);
	case 1: return to_variable(ecx);
	case 2: return to_variable(edx);
	case 3: return to_variable(ebx);
	case 4: return to_variable(esp);
	case 5: return to_variable(ebp);
	case 6: return to_variable(esi);
	case 7: return to_variable(edi);
	case 8: return to_variable(r8d);
	case 9: return to_variable(r9d);
	case 10: return to_variable(r10d);
	case 11: return to_variable(r11d);
	case 12: return to_variable(r12d);
	case 13: return to_variable(r13d);
	case 14: return to_variable(r14d);
	case 15: return to_variable(r15d);
		default: ensure(false);
	}
}

variable po::amd64::decode_reg64(unsigned int r_reg)
{
	switch(r_reg)
	{
	case 0: return to_variable(rax);
	case 1: return to_variable(rcx);
	case 2: return to_variable(rdx);
	case 3: return to_variable(rbx);
	case 4: return to_variable(rsp);
	case 5: return to_variable(rbp);
	case 6: return to_variable(rsi);
	case 7: return to_variable(rdi);
	case 8: return to_variable(r8);
	case 9: return to_variable(r9);
	case 10: return to_variable(r10);
	case 11: return to_variable(r11);
	case 12: return to_variable(r12);
	case 13: return to_variable(r13);
	case 14: return to_variable(r14);
	case 15: return to_variable(r15);
		default: ensure(false);
	}
}

po::variable po::amd64::select_reg(amd64_state::OperandSize os,unsigned int r,bool rex)
{
	switch(os)
	{
		case amd64_state::OpSz_8: return decode_reg8(r,rex);
		case amd64_state::OpSz_16: return decode_reg16(r);
		case amd64_state::OpSz_32: return decode_reg32(r);
		case amd64_state::OpSz_64: return decode_reg64(r);
		default: ensure(false);
	}
}

po::memory po::amd64::select_mem(amd64_state::OperandSize os,rvalue o)
{
	switch(os)
	{
		case amd64_state::OpSz_8: return byte(o);
		case amd64_state::OpSz_16: return word(o);
		case amd64_state::OpSz_32: return dword(o);
		case amd64_state::OpSz_64: return qword(o);
		default: ensure(false);
	}
}

po::lvalue po::amd64::decode_modrm(
		unsigned int mod,
		unsigned int b_rm,	// B.R/M
		boost::optional<constant> disp,
		boost::optional<std::tuple<unsigned int,unsigned int,unsigned int>> sib, // scale, X.index, B.base
		amd64_state::OperandSize os,
		amd64_state::AddressSize as,
		amd64_state::Mode mode,
		bool rex,
		cg& c)
{
	ensure(mod < 0x4);
	ensure(b_rm < 0x10);

	switch(as)
	{
		case amd64_state::AddrSz_16:
		{
			switch(mod)
			{
				case 0: case 1: case 2:
				{
					if(b_rm == 6)
					{
						if(mod == 0)
							return select_mem(os,*disp);
						else
							return c.add_i(select_mem(os,bp),*disp);
					}
					else
					{
						lvalue base = undefined();

						switch(b_rm)
						{
							case 0: base = select_mem(os,c.add_i(bx,si)); break;
							case 1: base = select_mem(os,c.add_i(bx,di)); break;
							case 2: base = select_mem(os,c.add_i(bp,si)); break;
							case 3: base = select_mem(os,c.add_i(bp,di)); break;
							case 4: base = select_mem(os,si); break;
							case 5: base = select_mem(os,di); break;
							case 7: base = select_mem(os,bx); break;
							default: ensure(false);
						}

						if(mod == 0)
							return base;
						else
							return c.add_i(base,*disp);
					}
				}

				case 3:
				{
					return select_reg(os,b_rm,rex);
				}

				default: ensure(false);
			}
		}

		case amd64_state::AddrSz_32:
		case amd64_state::AddrSz_64:
		{
			boost::optional<lvalue> base;

			switch(b_rm)
			{
				case 0: case 1: case 2: case 3:
				case 6: case 7: case 8: case 9: case 10: case 11:
				case 14: case 15:
					base = select_reg(mod != 3 && as == amd64_state::AddrSz_64 ? amd64_state::OpSz_64 : os,b_rm,rex);
					break;

				case 4: case 12:
					if(mod == 3)
					{
						base = select_reg(os,b_rm,rex);
						break;
					}
					else
						return decode_sib(mod,std::get<0>(*sib),std::get<1>(*sib),std::get<2>(*sib),disp,os,c);

				case 5:
				case 13:
					if(mod == 0)
					{
						if(mode == amd64_state::LongMode)
						{
							if(as == amd64_state::AddrSz_64)
								return select_mem(os,c.mod_i(c.add_i(*disp,rip),constant(0xffffffffffffffff)));
							else
								return select_mem(os,c.mod_i(c.add_i(*disp,eip),constant(0xffffffff)));
						}
						else
							return select_mem(os,*disp);
					}
					else
						base = select_reg(mod != 3 && as == amd64_state::AddrSz_64 ? amd64_state::OpSz_64 : os,b_rm,rex);
					break;

				default: ensure(false);
			}

			switch(mod)
			{
				case 0: return select_mem(os,*base);
				case 1: return select_mem(os,c.add_i(*base,*disp));
				case 2: return select_mem(os,c.add_i(*base,*disp));
				case 3: return *base;
				default: ensure(false);
			}
		}
		default: ensure(false);
	}
}

po::memory po::amd64::decode_sib(
	unsigned int mod,
	unsigned int scale,
	unsigned int x_index,
	unsigned int b_base,
	boost::optional<constant> disp,
	amd64_state::OperandSize os,
	cg& c)
{
	ensure(mod <= 3 && scale <= 3 && x_index <= 15 && b_base <= 15);

	switch(mod)
	{
		case 0:
		{
			switch(b_base)
			{
				case 0: case 1: case 2: case 3: case 4:
				case 6: case 7: case 8: case 9: case 10: case 11: case 12:
				case 14: case 15:
				{
					switch(x_index)
					{
						case 0: case 1: case 2: case 3:
						case 5: case 6: case 7: case 8: case 9: case 10: case 11: case 12: case 13: case 14: case 15:
						{
							variable base = decode_reg64(b_base);
							variable index = decode_reg64(x_index);

							if(scale > 0)
								return select_mem(os,c.add_i(base,c.mul_i(index,constant((1 << (scale & 3)) / 2))));
							else
								return select_mem(os,c.add_i(base,index));
						}
						case 4:
							return select_mem(os,constant(b_base & 7));
						default: ensure(false);
					}
				}
				case 5:
				case 13:
				{
					switch(x_index)
					{
						case 0: case 1: case 2: case 3:
						case 5: case 6: case 7: case 8: case 9: case 10: case 11: case 12: case 13: case 14: case 15:
						{
							variable index = decode_reg64(x_index);

							if(scale > 0)
								return select_mem(os,c.add_i(*disp,c.mul_i(index,constant((1 << (scale & 3)) / 2))));
							else
								return select_mem(os,c.add_i(*disp,index));
						}
						case 4:
							return select_mem(os,*disp);
						default: ensure(false);
					}
				}
				default: ensure(false);
			}
		}
		case 1:
		case 2:
		{
			switch(x_index)
			{
				case 0: case 1: case 2: case 3:
				case 5: case 6: case 7: case 8: case 9: case 10: case 11: case 12: case 13: case 14: case 15:
				{
					variable base = decode_reg64(b_base);
					variable index = decode_reg64(x_index);

					if(scale > 0)
						return select_mem(os,c.add_i(base,c.add_i(c.mul_i(index,constant((1 << (scale & 3)) / 2)),*disp)));
					else
						return select_mem(os,c.add_i(base,c.add_i(index,*disp)));
				}
				case 4:
					return select_mem(os,c.add_i(decode_reg64(b_base),*disp));
				default: ensure(false);
			}
		}
		default: ensure(false);
	}
}

std::pair<po::rvalue,po::rvalue> po::amd64::decode_rm(sm const& st,cg&)
{
	ensure(st.state.reg && st.state.rm);
	return std::make_pair(*st.state.reg,*st.state.rm);
}

std::pair<po::rvalue,po::rvalue> po::amd64::decode_sregm(sm const& st,cg&)
{
	ensure(st.state.reg && st.state.rm);
	ensure(is_variable(*st.state.reg));

	variable reg = to_variable(*st.state.reg);

	if(reg == ax || reg == eax)
		return std::make_pair(es,*st.state.rm);
	else if(reg == cx || reg == ecx)
		return std::make_pair(cs,*st.state.rm);
	else if(reg == dx || reg == edx)
		return std::make_pair(ss,*st.state.rm);
	else if(reg == bx || reg == ebx)
		return std::make_pair(ds,*st.state.rm);
	else if(reg == sp || reg == esp)
		return std::make_pair(fs,*st.state.rm);
	else if(reg == bp || reg == ebp)
		return std::make_pair(gs,*st.state.rm);
	else
		throw std::invalid_argument("unknown segment register");
}

std::pair<po::rvalue,po::rvalue> po::amd64::decode_msreg(sm const& st,cg& c)
{
	auto ret = decode_sregm(st,c);
	return std::make_pair(ret.second,ret.first);
}

std::pair<po::rvalue,po::rvalue> po::amd64::decode_ctrlrm(sm const& st,cg&)
{
	ensure(st.state.reg && st.state.rm);
	ensure(is_variable(*st.state.reg));

	variable reg = to_variable(*st.state.reg);

	if(reg == eax || reg == rax)
		return std::make_pair(cr0,*st.state.rm);
	else if(reg == edx || reg == rdx)
		return std::make_pair(cr2,*st.state.rm);
	else if(reg == ebx || reg == rbx)
		return std::make_pair(cr3,*st.state.rm);
	else if(reg == esp || reg == rsp)
		return std::make_pair(cr4,*st.state.rm);
	else if(reg == r9w || reg == r8)
		return std::make_pair(cr8,*st.state.rm);
	else
	{
		std::cerr << reg << std::endl;
		throw std::invalid_argument("unknown control register");
	}
}

std::pair<po::rvalue,po::rvalue> po::amd64::decode_rmctrl(sm const& st,cg& c)
{
	auto ret = decode_ctrlrm(st,c);
	return std::make_pair(ret.second,ret.first);
}

std::pair<po::rvalue,po::rvalue> po::amd64::decode_dbgrm(sm const& st,cg&)
{
	ensure(st.state.reg && st.state.rm);
	ensure(is_variable(*st.state.reg));

	variable reg = to_variable(*st.state.reg);

	if(reg == eax || reg == rax)
		return std::make_pair(dr0,*st.state.rm);
	else if(reg == ecx || reg == rcx)
		return std::make_pair(dr1,*st.state.rm);
	else if(reg == edx || reg == rdx)
		return std::make_pair(dr2,*st.state.rm);
	else if(reg == ebx || reg == rbx)
		return std::make_pair(dr3,*st.state.rm);
	else if(reg == esp || reg == rsp)
		return std::make_pair(dr4,*st.state.rm);
	else if(reg == ebp || reg == rbp)
		return std::make_pair(dr5,*st.state.rm);
	else if(reg == esi || reg == rsi)
		return std::make_pair(dr6,*st.state.rm);
	else if(reg == edi || reg == rdi)
		return std::make_pair(dr7,*st.state.rm);
	else
		throw std::invalid_argument("unknown debug register");
}

std::pair<po::rvalue,po::rvalue> po::amd64::decode_rmdbg(sm const& st,cg& c)
{
	auto ret = decode_dbgrm(st,c);
	return std::make_pair(ret.second,ret.first);
}
po::rvalue po::amd64::decode_rm1(sm const& st,cg&)
{
	ensure(st.state.rm);
	return *st.state.rm;
}

std::pair<rvalue,rvalue> po::amd64::decode_mr(sm const& st,cg&)
{
	ensure(st.state.reg && st.state.rm);
	return std::make_pair(*st.state.rm,*st.state.reg);
}

std::pair<rvalue,rvalue> po::amd64::decode_fd(sm const& st,cg&)
{
	ensure(st.state.moffs);
	return std::make_pair(select_reg(st.state.op_sz,0,st.state.rex),select_mem(st.state.op_sz,*st.state.moffs));
}

std::pair<rvalue,rvalue> po::amd64::decode_td(sm const& st,cg&)
{
	ensure(st.state.moffs);
	return std::make_pair(select_mem(st.state.op_sz,*st.state.moffs),select_reg(st.state.op_sz,0,st.state.rex));
}

std::pair<rvalue,rvalue> po::amd64::decode_mi(sm const& st,cg&)
{
	ensure(st.state.rm && st.state.imm);
	return std::make_pair(*st.state.rm,*st.state.imm);
}

std::pair<rvalue,rvalue> po::amd64::decode_i(sm const& st,cg&)
{
	ensure(st.state.imm);
	switch(st.state.op_sz)
	{
		case amd64_state::OpSz_8: return std::make_pair(ah,*st.state.imm);
		case amd64_state::OpSz_16: return std::make_pair(ax,*st.state.imm);
		case amd64_state::OpSz_32: return std::make_pair(eax,*st.state.imm);
		case amd64_state::OpSz_64: return std::make_pair(rax,*st.state.imm);
		default: ensure(false);
	}
}

rvalue po::amd64::decode_m(sm const& st,cg&)
{
	ensure(st.state.rm);
	return *st.state.rm;
}

rvalue po::amd64::decode_d(sm const& st,cg&)
{
	ensure(st.state.imm);

	constant c = *st.state.imm;
	uint64_t a,b;
	unsigned int s;

	if(c.content() <= 0xffffffff)
	{
		a = st.state.imm->content() >> 16;
		b = c.content() & 0xffff;
		s = 16;
	}
	else
	{
		a = st.state.imm->content() >> 32;
		b = c.content() & 0xffffffff;
		s = 32;
	}

	return constant((a << s) | b);
}

rvalue po::amd64::decode_imm(sm const& st,cg&)
{
	ensure(st.state.imm);
	return *st.state.imm;
}

rvalue po::amd64::decode_moffs(sm const& st,cg&)
{
	ensure(st.state.moffs);
	return select_mem(st.state.op_sz,*st.state.moffs);
}

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
		guard g(flag,relation::Eq,set ? constant(1) : constant(0));
		constant k((int8_t)(_k <= 63 ? _k : _k - 128));*/

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

std::pair<rvalue,rvalue> po::amd64::decode_ii(sm const& st,cg&)
{
	ensure(st.state.imm);
	return std::make_pair(constant(st.state.imm->content() >> 8),constant(st.state.imm->content() & 0xff));
}

std::tuple<rvalue,rvalue,rvalue> po::amd64::decode_rmi(sm const& st,cg&)
{
	ensure(st.state.reg && st.state.rm && st.state.imm);
	return std::make_tuple(*st.state.reg,*st.state.rm,*st.state.imm);
}

std::tuple<rvalue,rvalue,rvalue> po::amd64::decode_mri(sm const& st,cg&)
{
	ensure(st.state.reg && st.state.rm && st.state.imm);
	return std::make_tuple(*st.state.rm,*st.state.reg,*st.state.imm);
}
