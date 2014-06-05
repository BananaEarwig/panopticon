#include <vector>
#include <iostream>

#include <boost/variant.hpp>
#include <panopticon/value.hh>

#pragma once

namespace po
{
	template<typename Symbol, typename Domain, typename Codomain>
	struct unop
	{
		bool operator==(const unop<Symbol,Domain,Codomain>& o) const { return right == o.right; }
		rvalue right;
	};

	template<typename Symbol, typename Domain, typename Codomain>
	struct binop
	{
		bool operator==(const binop<Symbol,Domain,Codomain>& o) const { return right == o.right && left == o.left; }
		rvalue left;
		rvalue right;
	};

	template<typename Symbol, typename Domain, typename Codomain>
	struct naryop
	{
		bool operator==(const naryop<Symbol,Domain,Codomain>& o) const { return operands == o.operands; }
		std::vector<rvalue> operands;
	};

	struct logic_domain {};
	struct integer_domain {};
	struct rational_domain {};
	using universe_domain = boost::variant<logic_domain,integer_domain>;

	struct and_symbol {};
	struct or_symbol {};
	struct negation_symbol {};
	struct implication_symbol {};
	struct equivalence_symbol {};
	struct phi_symbol {};
	struct chi_symbol {};
	struct mu_symbol {};
	struct add_symbol {};
	struct subtract_symbol {};
	struct multiply_symbol {};
	struct divide_symbol {};
	struct modulo_symbol {};
	struct less_symbol {};
	struct equal_symbol {};
	struct lift_symbol {};
	struct call_symbol {};
	struct nop_symbol {};

	using logic_and = binop<and_symbol,logic_domain,logic_domain>;
	using logic_or = binop<or_symbol,logic_domain,logic_domain>;
	using logic_neg = unop<negation_symbol,logic_domain,logic_domain>;
	using logic_impl = binop<implication_symbol,logic_domain,logic_domain>;
	using logic_equiv = binop<equivalence_symbol,logic_domain,logic_domain>;

	using int_and = binop<and_symbol,integer_domain,integer_domain>;
	using int_or = binop<or_symbol,integer_domain,integer_domain>;
	using int_neg = unop<negation_symbol,integer_domain,integer_domain>;
	using int_add = binop<add_symbol,integer_domain,integer_domain>;
	using int_sub = binop<subtract_symbol,integer_domain,integer_domain>;
	using int_mul = binop<multiply_symbol,integer_domain,integer_domain>;
	using int_div = binop<divide_symbol,integer_domain,integer_domain>;
	using int_mod = binop<modulo_symbol,integer_domain,integer_domain>;
	using int_less = binop<less_symbol,integer_domain,logic_domain>;
	using int_equal = binop<equal_symbol,integer_domain,logic_domain>;
	using int_lift = unop<lift_symbol,logic_domain,integer_domain>;
	using int_call = unop<call_symbol,logic_domain,integer_domain>;

	using univ_phi = naryop<phi_symbol,universe_domain,universe_domain>;
	using univ_nop = unop<nop_symbol,universe_domain,universe_domain>;

	template<typename T>
	struct has_symbol_visitor : public boost::static_visitor<bool>
	{
		template<typename Domain,typename Codomain>
		bool operator()(unop<T,Domain,Codomain>) const { return true; }

		template<typename Domain,typename Codomain>
		bool operator()(binop<T,Domain,Codomain>) const { return true; }

		template<typename Domain,typename Codomain>
		bool operator()(naryop<T,Domain,Codomain>) const { return true; }

		template<typename Symbol,typename Domain,typename Codomain>
		bool operator()(unop<Symbol,Domain,Codomain>) const { return false; }

		template<typename Symbol,typename Domain,typename Codomain>
		bool operator()(binop<Symbol,Domain,Codomain>) const { return false; }

		template<typename Symbol,typename Domain,typename Codomain>
		bool operator()(naryop<Symbol,Domain,Codomain>) const { return false; }
	};

	/**
	 * @brief Single IL statement
	 *
	 * In order to allow code analysis algorithms to
	 * be implemented in a instruction set-agnostic manner,
	 * all opcodes are translated into a intermediate
	 * language first. Analysis is done on the IL and the
	 * results are mapped back to the original code.
	 *
	 * Every instance of the instr class models on IL statement.
	 * Each statement has the form a := f(b,...,z) where @c f is
	 * a @ref Function defined in the IL, @c b to @z its
	 * arguments (currently up to 3) and @c a is the variable
	 * receiving the result for @c f.
	 */
	struct instr
	{
		using operation = boost::variant<
			logic_and,
			logic_or,
			logic_neg,
			logic_impl,
			logic_equiv,
			univ_phi,
			univ_nop,
			int_and,
			int_or,
			int_neg,
			int_add,
			int_sub,
			int_mul,
			int_div,
			int_mod,
			int_less,
			int_equal,
			int_lift,
			int_call
		>;

		/// Construct a statement applying function @arg fn to @arg args. Saves the result in @arg a
		instr(const operation& op, const lvalue& a) : function(op), assignee(a) {}

		bool operator==(const instr& i) const { return function == i.function && assignee == i.assignee; }

		operation function;
		lvalue assignee;
	};

	std::vector<rvalue> operands(const instr&);
	void set_operands(instr&, const std::vector<rvalue>&);
	std::ostream& operator<<(std::ostream &os, const instr &i);

	template<>
	instr* unmarshal(const uuid&, const rdf::storage&);

	template<>
	rdf::statements marshal(const instr*, const uuid&);

	/// Pretty print the function
	std::string pretty(const instr::operation& fn);

	/// Returns a string suitable for describing the function in RDF
	std::string symbolic(const instr::operation& fn);

	/// Maps a string returned from @ref symbolic back the enum value
	instr::operation from_symbolic(const std::string &s, const std::vector<rvalue>&);
}
