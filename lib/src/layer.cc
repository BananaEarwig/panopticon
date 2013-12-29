#include <boost/iterator/counting_iterator.hpp>
#include <boost/iterator/zip_iterator.hpp>
#include <boost/iterator/transform_iterator.hpp>
#include <boost/tuple/tuple.hpp>
#include <panopticon/layer.hh>

using namespace po;
using namespace std;
using namespace boost;

po::layer_wloc po::operator+=(po::layer_wloc& a, const po::layer_wloc &b)
{
	return a = b;
}

map_layer::map_layer(const string &n, function<uint8_t(uint8_t)> fn)
: _name(n), _operation(fn)
{}

bool map_layer::operator==(const map_layer &a) const
{
	return a._name == _name;
}

slab map_layer::filter(const slab& in) const
{
	return adaptors::transform(in,adaptor(this));
}

const string& map_layer::name(void) const
{
	return _name;
}

map_layer::adaptor::adaptor(const map_layer *p) : parent(p) {}
uint8_t map_layer::adaptor::operator()(uint8_t i) const { return parent->_operation(i); }

anonymous_layer::anonymous_layer(std::initializer_list<byte> il, const std::string &n) : data(il), _name(n) {}
anonymous_layer::anonymous_layer(offset sz, const std::string &n) : data(sz), _name(n) {}

bool anonymous_layer::operator==(const anonymous_layer &a) const { return a.name() == name() && a.data == data; }

slab anonymous_layer::filter(const slab&) const { return slab(data.cbegin(),data.cend()); }
const std::string& anonymous_layer::name(void) const { return _name; }

mutable_layer::mutable_layer(const std::string &n) : data(), _name(n) {}

slab mutable_layer::filter(const slab& in) const
{
	auto b = make_zip_iterator(boost::make_tuple(counting_iterator<offset>(0),boost::begin(in)));
	auto e = make_zip_iterator(boost::make_tuple(counting_iterator<offset>(size(in)),boost::end(in)));
	auto fn = [this](const boost::tuples::tuple<offset,byte> &p) { return data.count(get<0>(p)) ? data.at(get<0>(p)) : get<1>(p); };
	return slab(make_transform_iterator(b,fn),make_transform_iterator(e,fn));
}

const std::string& mutable_layer::name(void) const { return _name; }

po::slab po::filter(const po::layer &a, const po::slab &s)
{
	if(boost::get<po::map_layer>(&a))
		return boost::get<po::map_layer>(a).filter(s);
	if(boost::get<po::mutable_layer>(&a))
		return boost::get<po::mutable_layer>(a).filter(s);
	if(boost::get<po::anonymous_layer>(&a))
		return boost::get<po::anonymous_layer>(a).filter(s);
	else
		throw invalid_argument("unknown layer type");
}

std::string po::name(const po::layer &a)
{
	if(boost::get<po::map_layer>(&a))
		return boost::get<po::map_layer>(a).name();
	if(boost::get<po::mutable_layer>(&a))
		return boost::get<po::mutable_layer>(a).name();
	if(boost::get<po::anonymous_layer>(&a))
		return boost::get<po::anonymous_layer>(a).name();
	else
		throw invalid_argument("unknown layer type");
}

stack::stack(void) : _graph(), _root(_graph.insert_node(layer_loc(uuids::random_generator()(),new layer(anonymous_layer({},"root"))))), _projection(none), _spanning_tree(none) {}
void stack::add(const bound &b, layer_loc l)
{
	auto proj = projection();
	auto i = proj.find(icl::first(b));
	auto vx = _graph.insert_node(l);
	bool t = false;

	while(i != proj.end() && icl::touches(bound(icl::first(i->first),icl::last(i->first) + 1),b))
	{
		bound n = bound(icl::first(i->first),icl::last(i->first) + 1) & b;
		_graph.insert_edge(n,vx,*_graph.find_node(i->second.lock()));

		++i;
		t = true;
	}

	if(!t)
		_graph.insert_edge(b,vx,_root);

	_projection = none;
	_spanning_tree = none;
}

const stack::image& stack::projection(void) const
{
	if(!_projection)
	{
		using vertex_descriptor = boost::graph_traits<digraph<layer_loc,bound>>::vertex_descriptor;
		using edge_descriptor = boost::graph_traits<digraph<layer_loc,bound>>::edge_descriptor;
		std::function<void(vertex_descriptor)> step;
		std::unordered_set<vertex_descriptor> visited;
		_projection = icl::split_interval_map<offset,layer_wloc>();

		step = [&](vertex_descriptor v)
		{
			layer_loc as = _graph.get_node(v);
			auto p = in_edges(v,_graph);

			assert(visited.insert(v).second);

			std::for_each(p.first,p.second,[&](edge_descriptor e)
			{
				bound b = _graph.get_edge(e);
				layer_loc other = _graph.get_node(source(e,_graph));

				*_projection += make_pair(b,layer_wloc(other));
			});

			std::for_each(p.first,p.second,[&](edge_descriptor e)
			{
				auto u = source(e,_graph);

				if(u != *_graph.nodes().second && !visited.count(u))
					step(u);
			});
		};

		//_projection = list<pair<bound,layer_wloc>>();
		step(_root);
		std::cerr << visited.size() << " " << _graph.num_nodes() << std::endl;
		assert(visited.size() == _graph.num_nodes());
	}

	return *_projection;
}

/*const icl::split_interval_map<offset,pair<bound,layer_wloc>> &stack::continuous(void) const
{
	if(!_continuous)
	{
		auto proj = projection();

		_continuous = icl::split_interval_map<offset,layer_wloc>();

		for(const pair<bound,layer_wloc> &p: *_projection)
		{
			offset b = icl::last(*_projection) + 1;
			_continuous->add(make_pair(bound(b,b + icl::size(p.first)),make_pair(p.first,layer_wloc(p.second))));
		}
	}

	return _continuous;
}*/

const stack::layers& stack::graph(void) const
{
	return _graph;
}

/*const stack::tree& stack::spanning_tree(void) const
{
	if(!_spanning_tree)
	{
		using vertex_descriptor = typename boost::graph_traits<digraph<po::layer_loc,po::bound>>::vertex_descriptor;
		using edge_descriptor = typename boost::graph_traits<digraph<po::layer_loc,po::bound>>::edge_descriptor;

		auto r = *_root;
		std::unordered_map<edge_descriptor,int> w_map;
		boost::associative_property_map<std::unordered_map<edge_descriptor,int>> weight_adaptor(w_map);
		auto common_parent = [&](vertex_descriptor v, vertex_descriptor u)
		{
			auto find_path = [&](vertex_descriptor x)
			{
				std::unordered_map<vertex_descriptor,vertex_descriptor> p_map;
				boost::associative_property_map<std::unordered_map<vertex_descriptor,vertex_descriptor>> pred_adaptor(p_map);

				boost::dijkstra_shortest_paths(g,x,boost::weight_map(weight_adaptor).predecessor_map(pred_adaptor));

				auto i = r;
				std::list<vertex_descriptor> path({i});
				while(i != p_map[i])
				{
					i = p_map[i];
					path.push_back(i);
				}
				return path;
			};

			auto l1 =	find_path(v);
			auto l2 = find_path(u);

			return *std::find_first_of(l1.begin(),l1.end(),l2.begin(),l2.end());
		};
		unordered_pmap<vertex_descriptor,vertex_descriptor> ret;

		for(auto v: iters(g.edges()))
			put(weight_adaptor,v,1);

		 *
		 * for(n: nodes(G))
		 * 	 for(e: in_edges(n))
		 *     c = source(e)
		 *     if(!in_tree(c))
		 *       add_to_tree(n,c)
		 *     else
		 *       del_from_tree(c)
		 *       add_to_tree(common_parent(n,c),c)
		 *
		auto revgraph = boost::make_reverse_graph(g);
		boost::breadth_first_search(revgraph,r,boost::visitor(boost::make_bfs_visitor(make_lambda_visitor(
			std::function<void(vertex_descriptor v)>([&](vertex_descriptor v)
			{
				for(auto e: iters(g.in_edges(v)))
				{
					auto c = source(e,g);
					if(ret.count(c) == 0)
						ret[c] = v;
					else
						ret[c] = common_parent(ret.at(c),v);
				}
			}),revgraph,boost::on_discover_vertex()))));

		_tree = unordered_map<layer_wloc,layer_wloc>();

		for(const pair<vertex_descriptor,vertex_descriptor> &p: ret)
			_tree.emplace(layer_wloc(_graph.get_node(p.first)),layer_wloc(_graph.get_node(p.second)));
	}

	return _tree;
}*/
