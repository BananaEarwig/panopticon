#include <algorithm>

#include <flowgraph.hh>
#include <basic_block.hh>

using namespace po;
using namespace std;

flowgraph::flowgraph(const string &n) : name(n) {}

odotstream &po::operator<<(odotstream &os, const flowgraph &f)
{
	os << "digraph G" << endl
		 << "{" << endl
		 << "\tgraph [compound=true,label=\"" << f.name << "\"];" << endl;

	cout << f.procedures.size() << " procs" << endl;

	for(proc_cptr p: f.procedures)
	{
		assert(p);

		if(os.body)
			os << subgraph << *p << nosubgraph << endl;
		else
			os << *p << endl;

		if(os.calls)
		{
			for(proc_cwptr q: p->callees)
			{
				auto qq = q.lock();

				if(qq && os.body)
				{
					cout << qq->entry << " " << p->entry << endl;
					if(qq->entry && p->entry)
					{
						os << "\t"
							 << unique_name(*p->entry) 
							 << " -> "
							 << unique_name(*qq->entry)
							 << " [lhead=cluster_" << unique_name(*qq)
							 << ",ltail=cluster_" << unique_name(*p)
							 << "];" << endl;
					}
				}
				else if(qq)
				{
					os << "\t" 
						 << unique_name(*p)
						 << " -> " 
						 << unique_name(*qq)
						 << ";" << endl;
				}
			}
		}
	}

	os << "}" << endl;
	return os;
}

flow_ptr flowgraph::unmarshal(const rdf::node &n, const rdf::storage &store)
{
	rdf::statement type = store.first(n,"type"_rdf,"Flowgraph"_po),
								 name = store.first(n,"name"_po,nullptr);
	rdf::stream procs = store.select(n,"include"_po,nullptr);
	flow_ptr ret(new flowgraph(name.object().to_string()));

	while(!procs.eof())
	{
		rdf::statement proc_node;

		procs >> proc_node;
		ret->procedures.insert(procedure::unmarshal(proc_node.object(),ret,store));
	}

	return ret;
}

string po::unique_name(const flowgraph &f)
{
	return "flow_" + to_string((uintptr_t)&f);
}

oturtlestream& po::operator<<(oturtlestream &os, const flowgraph &f)
{
	os << "[" << endl 
		 << " po:name \"" << f.name << "\"^^xsd:string;" << endl
		 << " rdf:type po:Flowgraph;" << endl;
	
	for(proc_cptr p: f.procedures)
		os << " po:include " << *p << ";" << endl;

	os << "]";
	return os;
}

ordfstream& po::operator<<(ordfstream &os, const flowgraph &f)
{
	rdf::node root;

	os.context().push(root);
	os << rdf::statement(root,"name"_po,rdf::lit(f.name));
	os << rdf::statement(root,"type"_rdf,"Flowgraph"_po);

	return os;
}

proc_ptr po::find_procedure(flow_ptr fg, addr_t a)
{
	std::set<proc_ptr>::iterator i = fg->procedures.begin();
	
	while(i != fg->procedures.end())
		if(find_bblock(*i,a))
			return *i;
		else 
			++i;
	
	return proc_ptr(0);
}

bool po::has_procedure(flow_ptr flow, addr_t entry)
{
	return any_of(flow->procedures.begin(),flow->procedures.end(),[&](const proc_ptr p) 
								{ return p && p->entry && p->entry->area().includes(entry); });
}

set<addr_t> po::collect_calls(proc_cptr proc)
{
	set<addr_t> ret;

	execute(proc,[&](const lvalue &left, instr::Function fn, const std::vector<rvalue> &right)
	{
		if(fn == instr::Call)
		{
			assert(right.size() == 1);

			if(right[0].is_constant())
			{
				const constant &c = right[0].to_constant();
				ret.insert(c.content());
			}
		}
	});

	return ret;
}
