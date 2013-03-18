#include <algorithm>
#include <functional>
#include <cassert>
#include <iostream>

#include <procedure.hh>
#include <flowgraph.hh>

using namespace po;
using namespace std;

domtree::domtree(bblock_ptr b) : intermediate(0), basic_block(b) {}

procedure::procedure(void) : name("unnamed") {}

void procedure::rev_postorder(function<void(bblock_ptr bb)> fn) const
{
	set<bblock_ptr> known;
	list<bblock_ptr> postorder;

	assert(entry);

	//cout << "rpo: " << basic_blocks.size() << ", entry: " << entry->area() << endl;
	//for_each(basic_blocks.begin(),basic_blocks.end(),[](const bblock_ptr bb) { cout << bb->area() << endl; });

	function<void(bblock_ptr)> visit;
	visit = [&](bblock_ptr bb)
	{
	//	cout << "visit " << bb->area() << endl;
		basic_block::succ_iterator i,iend;
		
		tie(i,iend) = bb->successors();
		for_each(i,iend,[&](bblock_ptr s)
		{	
		//	cout << "check " << s->area() << endl;
			if(known.insert(s).second)
				visit(s);
		});
		postorder.push_back(bb);
	};

	known.insert(entry);
	visit(entry);
	assert(basic_blocks.size() == postorder.size());
	for_each(postorder.rbegin(),postorder.rend(),fn);
}

bblock_ptr po::find_bblock(proc_ptr proc, addr_t a)
{
	auto i = proc->basic_blocks.begin();

	while(i != proc->basic_blocks.end())
	{
		bblock_ptr bb = *i++;
		
		if(bb->area().includes(a))
			return bb;
	}

	return bblock_ptr(0);
}

string po::graphviz(proc_ptr proc)
{
	flow_ptr f(new flowgraph());

	f->procedures.insert(proc);
	return graphviz(f);
}

void po::call(proc_ptr from, proc_ptr to)
{
	assert(from && to);

	from->callees.insert(to);
	to->callers.insert(from);
}

void po::execute(proc_cptr proc,function<void(const lvalue &left, instr::Function fn, const vector<rvalue> &right)> f)
{
	for(const bblock_ptr &bb: proc->basic_blocks)
	{
		size_t sz_mne = bb->mnemonics().size(), i_mne = 0;
		const mnemonic *ary_mne = bb->mnemonics().data();

		while(i_mne < sz_mne)
		{
			const mnemonic &mne = ary_mne[i_mne++];
			size_t sz_instr = mne.instructions.size(), i_instr = 0;
			const instr *ary_instr = mne.instructions.data();

			while(i_instr < sz_instr)
			{
				const instr &instr = ary_instr[i_instr++];

				f(instr.left,instr.function,instr.right);
			}
		}
	}
}
