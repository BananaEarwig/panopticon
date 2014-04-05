#include <memory>
#include <list>
#include <map>
#include <cassert>

#include <boost/iterator/iterator_facade.hpp>
#include <boost/iterator/filter_iterator.hpp>

#include <panopticon/marshal.hh>
#include <panopticon/loc.hh>

#pragma once

/**
 * @file
 * @brief Basic block structure and related classes
 *
 * A basic block is a sequence of mnemonics that are executed as a sequence.
 * The code of a function is partitioned into basic blocks. All its mnemonics are in one
 * continious block in memory and no mnemonic but the last is a jump instruction.
 *
 * Basic blocks have incoming and outgoing control transfers that model jump instructions.
 * Each control transfer has a source and a destination value and a guard. The guard is a
 * collection of conditions that are all true if this control transfer is taken. Both guards
 * and source/destination values can be symbolic i.e. variables or memory references. These
 * need to be evaluated in the context of the source basic block.
 */

namespace po
{
	struct basic_block;
	using bblock_loc = loc<basic_block>;
	using bblock_wloc = wloc<basic_block>;
}

#include <panopticon/mnemonic.hh>

namespace po
{
	/**
	 * @brief A logical relation.
	 */
	struct relation
	{
		enum Relcode
		{
			ULeq,
			SLeq,
			UGeq,
			SGeq,
			ULess,
			SLess,
			UGrtr,
			SGrtr,
			Eq,
			Neq,
		};

		/**
		 * Constructs a new relation with left side @ref a,
		 * right side @ref b and relation code @ref c.
		 */
		relation(rvalue a, Relcode c, rvalue b);

		Relcode relcode;
		rvalue operand1;	///< Left side
		rvalue operand2;	///< Right side
	};

	/**
	 * @brief Condition of a control transfer.
	 *
	 * A guard instance is a conjunction of relations that is
	 * true if a its associated control transfer is taken.
	 *
	 * @note A empty guard i.e. one w/o any relations is always true.
	 */
	struct guard
	{
		/// Constructs a guard that is always true
		guard(void);
		guard(const guard &g);
		guard(guard &&g);
		guard(const std::list<relation> &rels);
		guard(std::list<relation> &&rels);
		guard(rvalue a, relation::Relcode, rvalue b);

		guard &operator=(const guard &g);
		guard &operator=(guard &&g);

		/**
		 * @returns the negation of this guard
		 * @todo Only works with guards that have a single relation
		 */
		guard negation(void) const;

		std::list<relation> relations; ///< Conjunction of relations
	};

	std::string symbolic(relation::Relcode r);

	/// @returns The relation @ref r as UTF-8 string.
	std::string pretty(relation::Relcode r);
	std::ostream& operator<<(std::ostream &os, const guard &g);

	/**
	 * @brief A jump between two basic blocks.
	 *
	 * A control transfer is a jump from one address to another with
	 * an optional condition. The address can be any value (constant,
	 * memory, variables,...).
	 *
	 * If the target of the jump is known and has been disassembled
	 * the instance includes a pointer to the basic blocks that spans
	 * this address.
	 */
	struct ctrans
	{
		/// Jump to address @ref v under condition @ref g
		ctrans(guard g, rvalue v);

		/// Jump to basic block @ref b under condition @ref g
		ctrans(guard g, bblock_loc b);

		guard condition;
		rvalue value;
		bblock_wloc bblock;
	};

	/**
	 * @brief Sequence of mnemonics with no jumps inbetween.
	 *
	 * A basic block is a span of mnemonics that are executed in sequence.
	 */
	struct basic_block
	{
		basic_block(void);

		bound area(void) const;
		const std::vector<mnemonic>& mnemonics(void) const { return _mnemonics; }
		std::vector<mnemonic>& mnemonics(void) { _area = boost::none; return _mnemonics; }

	private:
		mutable boost::optional<bound> _area;
		std::vector<mnemonic> _mnemonics;
	};

	template<>
	rdf::statements marshal(const basic_block*, const uuid&);

	template<>
	basic_block* unmarshal(const uuid&, const rdf::storage&);

	/// Iterates all mnemoics in @ref bb, calling @ref f for each instruction.
	void execute(bblock_loc bb,std::function<void(const lvalue&,instr::Function,const std::vector<rvalue>&)> f);

	/// Iterates all mnemoics in @ref bb, calling @ref f for each instruction.
	void execute(bblock_loc bb,std::function<void(const instr&)> f);
}
