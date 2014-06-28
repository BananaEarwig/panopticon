#ifndef AVR_PRIVATE
#error "This header is not part of the public API!"
#endif

#include <panopticon/value.hh>

#pragma once

namespace po
{
	namespace avr
	{
		enum IndirectRegOp { PostInc, PreDec, PostDisplace, Nop };

		variable decode_reg(unsigned int r);
		variable decode_ioreg(unsigned int a);
		variable decode_preg(unsigned int r, IndirectRegOp op = Nop, int d = 0);

		memory sram(rvalue o);
		memory sram(unsigned int o);
		memory flash(rvalue o);
		memory flash(unsigned int o);

		sem_action unary_reg(::std::string x, ::std::function<void(cg &c, const variable &v)> func = ::std::function<void(cg&,const variable&)>());
		sem_action binary_reg(::std::string x, ::std::function<void(cg &,const variable&,const variable&)> func);
		sem_action branch(::std::string m, rvalue flag, bool set);
		sem_action binary_regconst(::std::string x, ::std::function<void(cg &,const variable&,const constant&)> func);
		sem_action binary_st(variable Rd1, variable Rd2, bool pre_dec = false, bool post_inc = false);
		sem_action binary_ld(variable Rr1, variable Rr2, bool pre_dec = false, bool post_inc = false);
		sem_action binary_stq(variable Rd1, variable Rd2);
		sem_action binary_ldq(variable Rr1, variable Rr2);
		sem_action simple(::std::string x, ::std::function<void(cg&)> fn);
	}
}
