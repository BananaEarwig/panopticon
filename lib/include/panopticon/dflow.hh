#include <memory>
#include <set>
#include <map>
#include <algorithm>

#include <panopticon/procedure.hh>
#include <panopticon/basic_block.hh>
#include <panopticon/tree.hh>

#pragma once

/**
 * @file
 * @brief Dataflow algorithms for program analysis
 *
 * Panopticon implements various classic dataflow algorithms:
 * - Basic Block dominance trees (\ref dominance_tree)
 * - Liveness analysis (\ref liveness)
 * - Static Single Assignment form (\ref ssa)
 *
 * All algorithms run on a per-procedure basis.
 */
namespace po
{
   /**
	* @brief Dominance tree of a procedure.
	*
	* The dominance tree is a tree of all basic blocks of a
	* procedure where the parent basic block occurs on all possible
	* paths from the entry point to this basic block.
	* The root of the tree is the basic block that includes the
	* entry point of a procedure.
	*/
	struct dom
	{
		tree<bblock_wloc> dominance;
		std::unordered_multimap<bblock_wloc,bblock_wloc> frontiers;
	};

	/**
	 * @brief Liveness information
	 *
	 * Holds the UEVar, VarKill and LiveOut sets for each
	 * basic block of a procedure, as well as global names
	 * (variables) and which blocks use them.
	 */
	struct live
	{
		std::unordered_set<std::string> names;										///< global (procedure-wide) names (ssa names w/o version)
		std::unordered_multimap<std::string,bblock_wloc> usage;		///< maps names to blocks that use them

		std::unordered_multimap<bblock_wloc,std::string> uevar;		///< up exposed variables
		std::unordered_multimap<bblock_wloc,std::string> varkill;	///< overwritten vars
		std::unordered_multimap<bblock_wloc,std::string> liveout;	///< live past the end
	};

	/// Computes a \ b
	template<typename T>
	std::set<T> set_difference(const std::set<T> &a, const std::set<T> &b)
	{
		std::set<T> ret;
		std::set_difference(a.begin(),a.end(),b.begin(),b.end(),std::inserter(ret,ret.begin()));
		return ret;
	}

	/// Computes a ∪ b
	template<typename T>
	std::set<T> set_union(const std::set<T> &a, const std::set<T> &b)
	{
		std::set<T> ret;
		//set_union(a.begin(),a.end(),b.begin(),b.end(),inserter(ret,ret.begin()));
		std::merge(a.begin(),a.end(),b.begin(),b.end(),std::inserter(ret,ret.begin()));
		return ret;
	}

	/// Computes a ∩ b
	template<typename T>
	std::set<T> set_intersection(const std::set<T> &a, const std::set<T> &b)
	{
		std::set<T> ret;
		std::set_intersection(a.begin(),a.end(),b.begin(),b.end(),std::inserter(ret,ret.begin()));
		return ret;
	}

	/// @brief Computes the dominance tree
	dom dominance_tree(proc_loc proc);

	/// @brief Transform the IL statements to SSA form
	void ssa(proc_loc proc, const dom &dominance, const live &liveness);

	/// @brief Computes liveness sets
	live liveness(proc_loc proc);
}
