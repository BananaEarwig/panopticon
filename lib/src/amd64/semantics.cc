#include <panopticon/amd64/amd64.hh>
#include <panopticon/amd64/semantics.hh>
#include <panopticon/code_generator.hh>

using namespace po;
using namespace dsl;

void po::amd64::do_push(rvalue _v, amd64_state::Mode mode, cg& m)
{
	variable v = to_variable(_v);
	int const w = v.width() / 8;

	m.assign(memory(rip,w,LittleEndian,"ram"),v);

	switch(mode)
	{
		case amd64_state::RealMode:
			m.assign(to_lvalue(sp),sp + w % 0x10000);
			return;
		case amd64_state::ProtectedMode:
			m.assign(to_lvalue(esp), esp + w % 0x100000000);
			return;
		case amd64_state::LongMode:
			m.assign(to_lvalue(rsp), rsp + w);
			return;
		default:
			throw std::invalid_argument("invalid mode in do_push");
	}
}

unsigned int po::amd64::bitwidth(rvalue a)
{
	if(is_variable(a))
		return to_variable(a).width();
	else if(is_memory(a))
		return to_memory(a).bytes() * 8;
	else
		throw std::invalid_argument("bitwidth() called with argument that is not a memory ref or variable.");
}

rvalue po::amd64::sign_ext(rvalue v, unsigned from, unsigned to, cg& m)
{
	using dsl::operator*;

	rvalue sign = v / (1 << (from - 1));
	rvalue rest = v % (1 << (from - 1));

	return (sign * (1 << (to - 1))) + rest;
}

void po::amd64::set_arithm_flags(rvalue res, rvalue res_half, rvalue a, rvalue b, cg& m)
{
	size_t const a_w = bitwidth(a);
	rvalue const msb_res = less(res / (1 << (a_w - 1)),1);

	m.assign(to_lvalue(CF),res / constant(1 << a_w));
	m.assign(to_lvalue(AF), res_half / constant(0x100));
	m.assign(to_lvalue(SF), msb_res);
	m.assign(to_lvalue(ZF), equal(a, constant(0)));
	m.assign(to_lvalue(OF), CF ^ SF);

	rvalue b0 = res % 2;
	rvalue b1 = (res % 4) / 2;
	rvalue b2 = (res % 8) / 4;
	rvalue b3 = (res % 16) / 8;
	rvalue b4 = (res % 32) / 16;
	rvalue b5 = (res % 64) / 32;
	rvalue b6 = (res % 128) / 64;
	rvalue b7 = (res % 256) / 128;
	m.assign(to_lvalue(PF), b0 ^ b1 ^ b2 ^ b3 ^ b4 ^ b5 ^ b6 ^ b7);
}

void po::amd64::adc(cg& m, rvalue a, rvalue b)
{
	size_t const a_w = bitwidth(a), b_w = (is_constant(b) ? a_w : bitwidth(b));
	rvalue const res = a + (a_w == b_w ? b : sign_ext(b,b_w,a_w,m)) + CF;
	rvalue const res_half = (a % constant(0x100)) + (b % constant(0x100)) + CF;

	m.assign(to_lvalue(a),res % constant(1 << a_w));
	set_arithm_flags(res,res_half,a,b,m);
}

void po::amd64::flagcomp(cg& m, variable const& flag)
{
	m.assign(flag,m.not_b(flag));
}

void po::amd64::flagwr(cg& m, variable const& flag,bool val)
{
	m.assign(flag,constant(!!val));
}

void po::amd64::aaa(cg& m)
{
	using dsl::operator*;

	rvalue y = al & constant(0x0f);
	rvalue x = m.or_b(m.not_b(m.or_b(less(y,constant(9)),equal(y,constant(9)))),AF);

	m.assign(to_lvalue(AF), m.lift_b(x));
	m.assign(to_lvalue(CF), m.lift_b(x));
	m.assign(to_lvalue(ax), (ax + m.lift_b(x) * constant(0x106)) % constant(0x100));
}

void po::amd64::aam(cg& m, rvalue a)
{
	rvalue temp_al = m.assign(to_lvalue(al));

	m.assign(to_lvalue(ah), temp_al / a);
	m.assign(to_lvalue(al), temp_al % a);
}

void po::amd64::aad(cg& m, rvalue a)
{
	using dsl::operator*;

	rvalue temp_al = m.assign(to_lvalue(al));
	rvalue temp_ah = m.assign(to_lvalue(ah));

	m.assign(to_lvalue(al), temp_al + temp_ah * a);
	m.assign(to_lvalue(ah), constant(0));
}

void po::amd64::aas(cg& m)
{
	using dsl::operator*;

	rvalue y = al & constant(0x0f);
	rvalue x = m.or_b(m.not_b(m.or_b(less(y,constant(9)),equal(y,constant(9)))),AF);

	m.assign(to_lvalue(AF), m.lift_b(x));
	m.assign(to_lvalue(CF), m.lift_b(x));
	m.assign(to_lvalue(ax), (ax - m.lift_b(x) * constant(6)) % constant(0x100));
	m.assign(to_lvalue(ah), (ah - m.lift_b(x)) % constant(0x10));
	m.assign(to_lvalue(al), y);
}

void po::amd64::add(cg& m, rvalue a, rvalue b)
{
	size_t const a_w = bitwidth(a), b_w = (is_constant(b) ? a_w : bitwidth(b));
	rvalue const res = a + (a_w == b_w ? b : sign_ext(b,b_w,a_w,m));
	rvalue const res_half = (a % constant(0x100)) + (b % constant(0x100));

	m.assign(to_lvalue(a),res % constant(1 << a_w));
	set_arithm_flags(res,res_half,a,b,m);
}

void po::amd64::adcx(cg& m, rvalue a, rvalue b)
{
	size_t const a_w = bitwidth(a);
	rvalue const res = a + b + CF;

	m.assign(to_lvalue(CF), res / constant(1 << a_w));
	m.assign(to_lvalue(a),res % constant(1 << a_w));
}

void po::amd64::and_(cg& m, rvalue a, rvalue b)
{
	unsigned int a_w = bitwidth(a), b_w = (is_constant(b) ? a_w : bitwidth(b));
	rvalue const res = a & (a_w == b_w ? b : sign_ext(b,b_w,a_w,m));
	rvalue const res_half = (a % constant(0x100)) & (b % constant(0x100));

	m.assign(to_lvalue(a),res);
	set_arithm_flags(res,res_half,a,b,m);
}

void po::amd64::arpl(cg& m, rvalue a, rvalue b) {}

void po::amd64::bound(cg& m, rvalue a, rvalue b) {}

void po::amd64::bsf(cg& m, rvalue a, rvalue b)
{
	using dsl::operator*;

	size_t const a_w = bitwidth(a);
	size_t bit = 0;
	boost::optional<rvalue> prev;

	m.assign(to_lvalue(ZF), equal(constant(0), b));

	while(bit < a_w)
	{
		rvalue val = (b % (1 << (bit + 1)) / (1 << bit));

		m.assign(to_lvalue(a),constant(bit + 1) * val);
		if(prev)
			prev = *prev | val;
		else
			prev = val;

		++bit;
	}
}

void po::amd64::bsr(cg& m, rvalue a, rvalue b)
{
	using dsl::operator*;

	size_t const a_w = bitwidth(a);
	size_t bit = a_w - 1;
	boost::optional<rvalue> prev;

	m.assign(to_lvalue(ZF), equal(constant(0), b));

	do
	{
		rvalue val = (b % (1 << (bit + 1)) / (1 << bit));

		m.assign(to_lvalue(a),constant(bit + 1) * val);
		if(prev)
			prev = *prev | val;
		else
			prev = val;
	}
	while(bit--);
}

void po::amd64::bswap(cg& m, rvalue a)
{
	using dsl::operator*;

	size_t const a_w = bitwidth(a);
	size_t byte = 0;

	rvalue tmp = undefined();

	while(byte < a_w / 8)
	{
		unsigned int lsb = byte * 8;
		unsigned int div = (1 << lsb), mul = (1 << (a_w - byte * 8));

		tmp = tmp + (((a / div) % constant(0x100)) * mul);
		++byte;
	}

	m.assign(to_lvalue(a),tmp);
}

void po::amd64::bt(cg& m, rvalue a, rvalue b)
{
	using dsl::operator<<;
	rvalue mod = (constant(1) << (b % constant(bitwidth(a))));

	m.assign(to_lvalue(CF), (a / mod) % 2);
	m.assign(to_lvalue(PF), undefined());
	m.assign(to_lvalue(OF), undefined());
	m.assign(to_lvalue(SF), undefined());
	m.assign(to_lvalue(AF), undefined());
}

void po::amd64::btc(cg& m, rvalue a, rvalue b)
{
	using dsl::operator<<;
	rvalue mod = (constant(1) << (b % constant(bitwidth(a))));

	m.assign(to_lvalue(CF), (a / mod) % 2);
	m.assign(to_lvalue(PF), undefined());
	m.assign(to_lvalue(OF), undefined());
	m.assign(to_lvalue(SF), undefined());
	m.assign(to_lvalue(AF), undefined());
	m.assign(to_lvalue(a),a ^ mod);
}

void po::amd64::btr(cg& m, rvalue a, rvalue b)
{
	using dsl::operator<<;
	size_t const a_w = bitwidth(a);
	rvalue mod =  ((constant(1) << (b % constant(bitwidth(a)))));

	m.assign(to_lvalue(CF), (a / mod) % 2);
	m.assign(to_lvalue(PF), undefined());
	m.assign(to_lvalue(OF), undefined());
	m.assign(to_lvalue(SF), undefined());
	m.assign(to_lvalue(AF), undefined());
	m.assign(to_lvalue(a),(a & (constant(0xffffffffffffffff) ^ mod)) % constant(1 << a_w));
}

void po::amd64::bts(cg& m, rvalue a, rvalue b)
{
	using dsl::operator<<;
	rvalue mod = (constant(1) << (b % constant(bitwidth(a))));

	m.assign(to_lvalue(CF), (a / mod) % 2);
	m.assign(to_lvalue(PF), undefined());
	m.assign(to_lvalue(OF), undefined());
	m.assign(to_lvalue(SF), undefined());
	m.assign(to_lvalue(AF), undefined());
	m.assign(to_lvalue(a),a & mod);
}

void po::amd64::near_call(cg& m, rvalue a, bool rel)
{
	rvalue new_ip;
	amd64_state::OperandSize op = amd64_state::OpSz_16;

	switch(op)
	{
		case amd64_state::OpSz_64:
		{
			if(rel)
				new_ip = (sign_ext(a,32,64,m) + rip);
			else
				new_ip = sign_ext(a,32,64,m);

			do_push(rip,amd64_state::LongMode,m);
			m.assign(to_lvalue(rip), new_ip);
			m.call_i(new_ip);

			return;
		}
		case amd64_state::OpSz_32:
		{
			if(rel)
				new_ip = (a + eip) % 0x100000000;
			else
				new_ip = a;

			do_push(eip,amd64_state::ProtectedMode,m);
			m.assign(to_lvalue(eip), new_ip);
			m.call_i(new_ip);

			return;
		}
		case amd64_state::OpSz_16:
		{
			if(rel)
				new_ip = (a + eip) % 0x10000;
			else
				new_ip = a % 0x10000;

			do_push(ip,amd64_state::RealMode,m);
			m.assign(to_lvalue(ip), new_ip);
			m.call_i(new_ip);

			return;
		}
		default:
			throw std::invalid_argument("near_call with wrong mode");
	}
}

void po::amd64::far_call(cg& m, rvalue a, bool rel)
{
	amd64_state::OperandSize op = amd64_state::OpSz_16;

	switch(op)
	{
		case amd64_state::OpSz_16:
		{
			do_push(cs,amd64_state::RealMode,m);
			do_push(ip,amd64_state::RealMode,m);

			return;
		}
		case amd64_state::OpSz_32:
		{
			do_push(cs,amd64_state::ProtectedMode,m);
			do_push(eip,amd64_state::ProtectedMode,m);

			return;
		}
		case amd64_state::OpSz_64:
		{
			do_push(cs,amd64_state::LongMode,m);
			do_push(rip,amd64_state::LongMode,m);

			return;
		}
		default:
			throw std::invalid_argument("far_call invalid op size");
	}
}

void po::amd64::cmov(cg& m, rvalue a, rvalue b, condition c)
{
	using dsl::operator*;

	auto fun = [&](rvalue f)
	{
		m.assign(to_lvalue(a),b + (m.lift_b(f) * b) + (m.lift_b(m.not_b(f)) * a));
	};

	switch(c)
	{
		case Overflow:    fun(OF); break;
		case NotOverflow: fun(m.not_b(OF)); break;
		case Carry:       fun(CF); break;
		case AboveEqual:  fun(m.not_b(CF)); break;
		case Equal:       fun(ZF); break;
		case NotEqual:    fun(m.not_b(ZF)); break;
		case BelowEqual:  fun(m.or_b(ZF,CF)); break;
		case Above:       fun(m.not_b(m.or_b(ZF,CF))); break;
		case Sign:        fun(SF); break;
		case NotSign:     fun(m.not_b(SF)); break;
		case Parity:      fun(PF); break;
		case NotParity:   fun(m.not_b(PF)); break;
		case Less:        fun(m.or_b(m.and_b(SF,OF),m.and_b(m.not_b(SF),m.not_b(OF)))); break;
		case GreaterEqual:fun(m.or_b(m.and_b(m.not_b(SF),OF),m.and_b(SF,m.not_b(OF)))); break;
		case LessEqual:   fun(m.or_b(ZF,m.or_b(m.and_b(SF,OF),m.and_b(m.not_b(SF),m.not_b(OF))))); break;
		case Greater:     fun(m.or_b(m.not_b(ZF),m.or_b(m.and_b(m.not_b(SF),OF),m.and_b(SF,m.not_b(OF))))); break;
		default:
			throw std::invalid_argument("invalid condition in cmov");
	}
}

void po::amd64::cmp(cg& m, rvalue a, rvalue b)
{
	unsigned int a_w = bitwidth(a), b_w = (is_constant(b) ? a_w : bitwidth(b));
	rvalue const res = a - (a_w == b_w ? b : sign_ext(b,b_w,a_w,m));
	rvalue const res_half = (a % constant(0x100)) - (b % constant(0x100));

	set_arithm_flags(res,res_half,a,b,m);
}

void po::amd64::cmps(cg& m, rvalue aoff, rvalue boff)
{
	using dsl::operator*;
	int bits = 8;

	rvalue const a = memory(aoff,bits / 8,LittleEndian,"ram"), b = memory(boff,bits / 8,LittleEndian,"ram");
	rvalue const res = a - b;
	rvalue const res_half = (a % 0x100) - (b % 0x100);

	set_arithm_flags(res,res_half,a,b,m);

	rvalue off = (bits / 8) * m.lift_b(DF) - (bits / 8) * m.lift_b(m.not_b(DF));

	m.assign(to_lvalue(aoff),aoff + off);
	m.assign(to_lvalue(boff),boff + off);
}

void po::amd64::cmpxchg(cg& m, rvalue a, rvalue b)
{
	using dsl::operator*;
	rvalue acc = eax;

	rvalue t = equal(a,acc);

	m.assign(to_lvalue(ZF), t);
	m.assign(to_lvalue(a),m.lift_b(t) * b + m.lift_b(m.not_b(t)) * a);
	m.assign(to_lvalue(acc),m.lift_b(t) * acc + m.lift_b(m.not_b(ZF)) * a);
}

void po::amd64::or_(cg& m, rvalue a, rvalue b)
{
	unsigned int a_w = bitwidth(a), b_w = (is_constant(b) ? a_w : bitwidth(b));
	rvalue const res = a | (a_w == b_w ? b : sign_ext(b,b_w,a_w,m));
	rvalue const res_half = (a % constant(0x100)) | (b % constant(0x100));

	m.assign(to_lvalue(a),res);
	set_arithm_flags(res,res_half,a,b,m);
}

void po::amd64::sbb(cg& m, rvalue a, rvalue b)
{
	unsigned int a_w = bitwidth(a), b_w = (is_constant(b) ? a_w : bitwidth(b));
	rvalue const res = a - (a_w == b_w ? b : sign_ext(b,b_w,a_w,m));
	rvalue const res_half = (a % constant(0x100)) - (b % constant(0x100)) - CF;

	m.assign(to_lvalue(a),res % constant(1 << a_w));
	set_arithm_flags(res,res_half,a,b,m);
}

void po::amd64::sub(cg& m, rvalue a, rvalue b)
{
	unsigned int a_w = bitwidth(a), b_w = (is_constant(b) ? a_w : bitwidth(b));
	rvalue const res = a - (a_w == b_w ? b : sign_ext(b,b_w,a_w,m));
	rvalue const res_half = (a % constant(0x100)) - (b % constant(0x100));

	m.assign(to_lvalue(a),res % constant(1 << a_w));
	set_arithm_flags(res,res_half,a,b,m);
}

void po::amd64::xor_(cg& m, rvalue a, rvalue b)
{
	unsigned int a_w = bitwidth(a), b_w = (is_constant(b) ? a_w : bitwidth(b));
	rvalue const res = a ^ (a_w == b_w ? b : sign_ext(b,b_w,a_w,m));
	rvalue const res_half = (a % constant(0x100)) ^ (b % constant(0x100));

	m.assign(to_lvalue(a),res);
	set_arithm_flags(res,res_half,a,b,m);
}

void po::amd64::cmpxchg8b(cg& m, rvalue a) {}
void po::amd64::cmpxchg16b(cg& m, rvalue a) {}
void po::amd64::cpuid(cg&) {}

bool po::amd64::conv(sm& st)
{
	st.jump(st.address + st.tokens.size());
	return true;
}

bool po::amd64::conv2(sm& st)
{
	st.jump(st.address + st.tokens.size());
	return true;
}

void po::amd64::daa(cg&) {}
void po::amd64::das(cg&) {}
void po::amd64::dec(cg& m, rvalue a) {}
void po::amd64::div(cg& m, rvalue a) {}
void po::amd64::enter(cg& m, rvalue a, rvalue b) {}
void po::amd64::hlt(cg&) {}
void po::amd64::idiv(cg&,rvalue) {}
void po::amd64::imul1(cg& m, rvalue a) {}
void po::amd64::imul2(cg& m, rvalue a, rvalue b) {}
void po::amd64::imul3(cg& m, rvalue a, rvalue b, rvalue c) {}
void po::amd64::in(cg& m, rvalue a, rvalue b) {}
void po::amd64::icebp(cg& m) {}
void po::amd64::inc(cg& m, rvalue a) {}
void po::amd64::ins(cg& m, rvalue a, rvalue b) {}
void po::amd64::int_(cg& m, rvalue a) {}
void po::amd64::into(cg& m) {}

bool po::amd64::iret(sm& st)
{
	st.jump(st.address + st.tokens.size());
	return true;
}

void po::amd64::jcc(cg&,rvalue a, condition c) {}
void po::amd64::jmp(cg&,rvalue a) {}
void po::amd64::jxz(cg&,rvalue a, rvalue b) {}
void po::amd64::lahf(cg& m) {}
void po::amd64::lar(cg& m, rvalue a, rvalue b) {}
void po::amd64::lxs(cg& m,rvalue a, rvalue b, rvalue seg) {}
void po::amd64::lea(cg& m,rvalue a, rvalue b) {}

bool po::amd64::leave(sm& st)
{
	st.jump(st.address + st.tokens.size());
	return true;
}

bool po::amd64::lodsb(sm& st)
{
	st.jump(st.address + st.tokens.size());
	return true;
}

bool po::amd64::lods(sm& st)
{
	st.jump(st.address + st.tokens.size());
	return true;
}

bool po::amd64::loop(sm& st)
{
	st.jump(st.address + st.tokens.size());
	return true;
}

bool po::amd64::loope(sm& st)
{
	st.jump(st.address + st.tokens.size());
	return true;
}

bool po::amd64::loopne(sm& st)
{
	st.jump(st.address + st.tokens.size());
	return true;
}

void po::amd64::mov(cg&,rvalue a,rvalue b,bool sign_ext) {}
void po::amd64::movbe(cg&,rvalue a,rvalue b) {}

bool po::amd64::movsb(sm& st)
{
	st.jump(st.address + st.tokens.size());
	return true;
}

bool po::amd64::movs(sm& st)
{
	st.jump(st.address + st.tokens.size());
	return true;
}

void po::amd64::movsx(cg&,rvalue a,rvalue b) {}
void po::amd64::movzx(cg&,rvalue a,rvalue b) {}
void po::amd64::mul(cg& m, rvalue a) {}
void po::amd64::neg(cg& m, rvalue a) {}
void po::amd64::nop(cg& m) {}
void po::amd64::not_(cg& m,rvalue) {}
void po::amd64::out(cg& m, rvalue a, rvalue b) {}

bool po::amd64::outs(sm& st)
{
	st.jump(st.address + st.tokens.size());
	return true;
}

bool po::amd64::pop(sm& st)
{
	st.jump(st.address + st.tokens.size());
	return true;
}

bool po::amd64::popa(sm& st)
{
	st.jump(st.address + st.tokens.size());
	return true;
}

void po::amd64::popcnt(cg& m, rvalue a, rvalue b) {}
void po::amd64::popf(cg& m,rvalue) {}

bool po::amd64::push(sm& st)
{
	st.jump(st.address + st.tokens.size());
	return true;
}

bool po::amd64::pusha(sm& st)
{
	st.jump(st.address + st.tokens.size());
	return true;
}

void po::amd64::pushf(cg& m,rvalue) {}
void po::amd64::rcl(cg& m, rvalue a, rvalue b) {}
void po::amd64::rcr(cg& m, rvalue a, rvalue b) {}
void po::amd64::ret(cg& m, rvalue a) {}
void po::amd64::retf(cg& m, rvalue a) {}
void po::amd64::ror(cg& m, rvalue a, rvalue b) {}
void po::amd64::rol(cg& m, rvalue a, rvalue b) {}
void po::amd64::sahf(cg& m) {}
void po::amd64::sal(cg& m, rvalue a, rvalue b) {}
void po::amd64::salc(cg& m) {}
void po::amd64::sar(cg& m, rvalue a, rvalue b) {}

bool po::amd64::scas(sm& st)
{
	st.jump(st.address + st.tokens.size());
	return true;
}

void po::amd64::setcc(cg& m, rvalue a, condition c) {}
void po::amd64::shl(cg& m, rvalue a, rvalue b) {}
void po::amd64::shr(cg& m, rvalue a, rvalue b) {}
void po::amd64::shld(cg& m, rvalue a, rvalue b, rvalue c) {}
void po::amd64::shrd(cg& m, rvalue a, rvalue b, rvalue c) {}

bool po::amd64::stos(sm& st)
{
	st.jump(st.address + st.tokens.size());
	return true;
}

void po::amd64::test(cg& m,rvalue a, rvalue b) {}
void po::amd64::ud1(cg& m) {}
void po::amd64::ud2(cg& m) {}
void po::amd64::xadd(cg& m, rvalue a, rvalue b) {}
void po::amd64::xchg(cg& m, rvalue a, rvalue b) {}
