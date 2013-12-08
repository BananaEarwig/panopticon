#include <iostream>
#include <vector>
#include <functional>
#include <cassert>
#include <unordered_map>

#include <graph.hh>

#include <boost/icl/discrete_interval.hpp>
#include <boost/icl/right_open_interval.hpp>
#include <boost/icl/interval_map.hpp>
#include <boost/icl/split_interval_map.hpp>

#pragma once

namespace po
{
	using rrange = typename boost::icl::interval<uint32_t>::type;

	typedef uint32_t addr_t;
	extern const addr_t naddr;

	// boost icl
	template<typename T>
	struct range
	{
		range(void) : begin(0), end(0) {}
		range(T b) : begin(b), end(b) {}
		range(T b, T e) : begin(b), end(e) { assert(begin <= end); }

		size_t size(void) const { return end - begin; }
		bool includes(const range<T> &a) const { return size() && a.size() && begin <= a.begin && end > a.end; }
		bool includes(T a) const { return size() && begin <= a && end > a; }
		bool overlap(const range<T> &a) const { return size() && a.size() && !(begin >= a.end || end <= a.begin); }
		T last(void) const { return size() ? end - 1 : begin; }

		bool operator==(const range<T> &b) const { return size() == b.size() && (!size() || (begin == b.begin && end == b.end)); }
		bool operator!=(const range<T> &b) const { return !(*this == b); }
		bool operator<(const range<T> &b) const { return begin < b.begin; }

		T begin;
		T end;
	};

	template<typename T>
	std::ostream& operator<<(std::ostream &os, const range<T> &r)
	{
		if(r.size())
		{
			os << r.begin;
			if(r.size() > 1)
				os << "-" << r.end-1;
		}
		else
			os << "nil";
		return os;
	}
}

namespace std
{
	template<>
	struct hash<po::rrange>
	{
		size_t operator()(const po::rrange &r) const
		{
			return hash<uint32_t>()(first(r)) xor hash<uint32_t>()(last(r));
		}
	};
}

namespace po
{
	struct address_space
	{
		using bytes = std::vector<uint8_t>;

		address_space(void) : name(), area(), m_map() {}
		address_space(const std::string &n, const rrange &a, std::function<bytes(const bytes&)> fn) : name(n), area(a), m_map(fn) {}
		bytes map(const bytes &bs, const rrange &a) const;

		bool operator==(const address_space &as) const { return name == as.name && area == as.area; }

		std::string name;
		rrange area;

	private:
		std::function<bytes(const bytes&)> m_map;

		friend struct std::hash<address_space>;
	};

	using as_ptr = std::shared_ptr<address_space>;
}

namespace std
{
	template<>
	struct hash<po::address_space>
	{
		size_t operator()(const po::address_space &as) const
		{
			return hash<string>()(as.name);
		}
	};

	template<typename X, typename Y>
	struct hash<pair<X,Y>>
	{
		size_t operator()(const std::pair<X,Y> &p) const
		{
			return std::hash<X>()(p.first) xor std::hash<Y>()(p.second);
		}
	};
}

namespace po
{
	struct structure
	{
		std::string name;
		std::string value;

		uint32_t reference;
		std::list<structure> fields;
	};

	using struct_ptr = std::shared_ptr<structure>;

	struct beacon
	{
		virtual ~beacon(void);
		std::string type(void) const;
	};

	struct metadata
	{
		enum Type
		{
			BasicBlockType,
			StructureType,
			RawType
		};

		Type type;
		union
		{
			bblock_ptr bb;
			struct_ptr st;
		};
	};

	struct datagraph
	{
		digraph<as_ptr,rrange> hierarchy;

		// onto projection of 'hierarchy'
		std::multimap<uint32_t,beacon> beacons;
		boost::icl::interval_map<uint32_t,metadata> meta;
	};

	using data_ptr = std::shared_ptr<datagraph>;

	std::list<std::pair<rrange,address_space>> projection(const address_space &as, const graph<address_space,rrange> &g);
	po::unordered_pmap<boost::graph_traits<po::graph<po::address_space,po::rrange>>::vertex_descriptor,boost::graph_traits<po::graph<po::address_space,po::rrange>>::vertex_descriptor>
	tree(const graph<address_space,rrange> &g);
	boost::graph_traits<po::graph<po::address_space,po::rrange>>::vertex_descriptor root(const graph<address_space,rrange> &g);
}
