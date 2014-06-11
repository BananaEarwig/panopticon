#include <panopticon/instr.hh>

using namespace po;

struct set_operands_visitor : public boost::static_visitor<>
{
	set_operands_visitor(const std::vector<rvalue>& rv) : boost::static_visitor<>(), _values(rv) {}

	template<typename Symbol, typename Domain, typename Codomain>
	void operator()(unop<Symbol,Domain,Codomain,rvalue>& op)
	{
		assert(_values.size() == 1);
		op.right = _values[0];
	}

	template<typename Symbol, typename Domain, typename Codomain>
	void operator()(binop<Symbol,Domain,Codomain,rvalue>& op)
	{
		assert(_values.size() == 2);
		op.left = _values[0];
		op.right = _values[1];
	}

	template<typename Symbol, typename Domain, typename Codomain>
	void operator()(naryop<Symbol,Domain,Codomain,rvalue>& op)
	{
		op.operands = _values;
	}

	const std::vector<rvalue>& _values;
};

void po::set_operands(instr& i, const std::vector<rvalue>& rv)
{
	set_operands_visitor vis(rv);
	boost::apply_visitor(vis,i.function);
}

struct operands_visitor : public boost::static_visitor<std::vector<rvalue>>
{
	template<typename Symbol, typename Domain, typename Codomain>
	result_type operator()(const unop<Symbol,Domain,Codomain,rvalue>& op) const
	{
		return {op.right};
	}

	template<typename Symbol, typename Domain, typename Codomain>
	result_type operator()(const binop<Symbol,Domain,Codomain,rvalue>& op) const
	{
		return {op.left,op.right};
	}

	template<typename Symbol, typename Domain, typename Codomain>
	result_type operator()(const naryop<Symbol,Domain,Codomain,rvalue>& op) const
	{
		return op.operands;
	}
};

std::vector<rvalue> po::operands(const instr& i)
{
	operands_visitor vis;
	return apply_visitor(vis,i.function);
}

std::string po::pretty(const instr::operation& i)
{
	struct vis : public boost::static_visitor<std::string>
	{
		std::string operator()(const logic_and<rvalue>&) const { return "∧"; }
		std::string operator()(const logic_or<rvalue>&) const { return "∨"; }
		std::string operator()(const logic_neg<rvalue>&) const { return "¬"; }
		std::string operator()(const logic_impl<rvalue>&) const { return "→"; }
		std::string operator()(const logic_equiv<rvalue>&) const { return "↔"; }

		std::string operator()(const int_and<rvalue>&) const { return "∧"; }
		std::string operator()(const int_or<rvalue>&) const { return "∨"; }
		std::string operator()(const int_neg<rvalue>&) const { return "¬"; }
		std::string operator()(const int_add<rvalue>&) const { return "+"; }
		std::string operator()(const int_sub<rvalue>&) const { return "-"; }
		std::string operator()(const int_mul<rvalue>&) const { return "×"; }
		std::string operator()(const int_div<rvalue>&) const { return "÷"; }
		std::string operator()(const int_mod<rvalue>&) const { return "%"; }
		std::string operator()(const int_less<rvalue>&) const { return "<"; }
		std::string operator()(const int_equal<rvalue>&) const { return "="; }
		std::string operator()(const int_lift<rvalue>&) const { return "int "; }
		std::string operator()(const int_call<rvalue>&) const { return "call "; }

		std::string operator()(const univ_phi<rvalue>&) const { return "ϕ"; }
		std::string operator()(const univ_nop<rvalue>&) const { return ""; }
	};
	vis v;

	return boost::apply_visitor(v,i);
}

std::string po::symbolic(const instr::operation& i)
{
	struct vis : public boost::static_visitor<std::string>
	{
		std::string operator()(const logic_and<rvalue>&) const { return "logic-and"; }
		std::string operator()(const logic_or<rvalue>&) const { return "logic-or"; }
		std::string operator()(const logic_neg<rvalue>&) const { return "logic-negation"; }
		std::string operator()(const logic_impl<rvalue>&) const { return "logic-implication"; }
		std::string operator()(const logic_equiv<rvalue>&) const { return "logic-equivalence"; }

		std::string operator()(const int_and<rvalue>&) const { return "integer-bitwise-and"; }
		std::string operator()(const int_or<rvalue>&) const { return "integer-bitwise-or"; }
		std::string operator()(const int_neg<rvalue>&) const { return "integer-bitwise-negation"; }
		std::string operator()(const int_add<rvalue>&) const { return "integer-addition"; }
		std::string operator()(const int_sub<rvalue>&) const { return "integer-subtraction"; }
		std::string operator()(const int_mul<rvalue>&) const { return "integer-multiplication"; }
		std::string operator()(const int_div<rvalue>&) const { return "integer-division"; }
		std::string operator()(const int_mod<rvalue>&) const { return "integer-modulo"; }
		std::string operator()(const int_less<rvalue>&) const { return "integer-less-than"; }
		std::string operator()(const int_equal<rvalue>&) const { return "integer-equal-to"; }
		std::string operator()(const int_lift<rvalue>&) const { return "integer-lift-boolean"; }
		std::string operator()(const int_call<rvalue>&) const { return "integer-call-to"; }

		std::string operator()(const univ_phi<rvalue>&) const { return "universal-phi"; }
		std::string operator()(const univ_nop<rvalue>&) const { return "universal-no-op"; }
	};
	vis v;

	return std::string(PO) + boost::apply_visitor(v,i);
}

instr::operation po::from_symbolic(const std::string &s, const std::vector<rvalue>& rv)
{
	if(s.substr(0,std::string(PO).size()) == std::string(PO))
	{
		std::string t = s.substr(std::string(PO).size());

		if(t == "logic-and") return logic_and<rvalue>{rv[0],rv[1]};
		if(t == "logic-or") return logic_or<rvalue>{rv[0],rv[1]};
		if(t == "logic-negation") return logic_neg<rvalue>{rv[0]};
		if(t == "logic-implication") return logic_impl<rvalue>{rv[0],rv[1]};
		if(t == "logic-equivalence") return logic_equiv<rvalue>{rv[0],rv[1]};

		if(t == "integer-bitwise-and") return int_and<rvalue>{rv[0],rv[1]};
		if(t == "integer-bitwise-or") return int_or<rvalue>{rv[0],rv[1]};
		if(t == "integer-bitwise-negation") return int_neg<rvalue>{rv[0]};
		if(t == "integer-addition") return int_add<rvalue>{rv[0],rv[1]};
		if(t == "integer-subtraction") return int_sub<rvalue>{rv[0],rv[1]};
		if(t == "integer-multiplication") return int_mul<rvalue>{rv[0],rv[1]};
		if(t == "integer-division") return int_div<rvalue>{rv[0],rv[1]};
		if(t == "integer-modulo") return int_mod<rvalue>{rv[0],rv[1]};
		if(t == "integer-less-than") return int_less<rvalue>{rv[0],rv[1]};
		if(t == "integer-equal-to") return int_equal<rvalue>{rv[0],rv[1]};
		if(t == "integer-lift-boolean") return int_lift<rvalue>{rv[0]};
		if(t == "integer-call-to") return int_call<rvalue>{rv[0]};

		if(t == "universal-phi") return univ_phi<rvalue>{rv};
		if(t == "universal-no-op") return univ_nop<rvalue>{rv[0]};
	}
	throw std::runtime_error("invalid string");
}

std::ostream &po::operator<<(std::ostream &os, const instr &i)
{
	std::string fnname = pretty(i.function);
	std::vector<rvalue> right = operands(i);

	os << i.assignee << " ≔ ";
	if(right.size() == 0)
		os << fnname;
	else if(boost::apply_visitor(has_symbol_visitor<call_symbol>(),i.function))
		os << fnname << "(" << right[0] << ")";
	else if(right.size() == 1)
		os << fnname << right[0];
	else if(boost::apply_visitor(has_symbol_visitor<phi_symbol>(),i.function))
		os << fnname << "(" << right[0] << ", " << right[1] << ")";
	else if(right.size() == 3)
		os << fnname << "(" << right[0] << ", " << right[1] << ", " << right[2] << ")";
	else
		os << right[0] << fnname << right[1];
	return os;
}
