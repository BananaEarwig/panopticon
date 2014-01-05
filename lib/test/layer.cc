#include <iostream>

#include <gtest/gtest.h>
#include <boost/range/algorithm/copy.hpp>
#include <panopticon/layer.hh>

using namespace po;
using namespace std;

TEST(layer,map_layer)
{
	layer l1 = map_layer("add 1",[](uint8_t i) { return i + 1; });
	vector<byte> d = {1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16}, r, e({2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17});

	boost::copy(filter(l1,slab(d)),back_inserter(r));
	ASSERT_EQ(r, e);
}

TEST(layer,anonymous_layer)
{
	layer l1 = anonymous_layer(128,"anon 1");
	layer l2 = anonymous_layer({1,2,3,4,5,6},"anon 2");
	vector<byte> r;

	ASSERT_EQ(128,boost::size(filter(l1,slab())));
	ASSERT_EQ(6,boost::size(filter(l2,slab())));

	boost::copy(filter(l2,slab()),back_inserter(r));
	ASSERT_EQ(r,vector<byte>({1,2,3,4,5,6}));
}

TEST(layer,mutable_layer)
{
	vector<byte> d = {1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16}, r, e({1,2,3,4,5,0,0,8,9,10,11,12,13,0,15,16});
	mutable_layer l1("mut");

	l1.data[5] = 0;
	l1.data[6] = 0;
	l1.data[13] = 0;

	boost::copy(filter(l1,slab(d)),back_inserter(r));
	ASSERT_EQ(r, e);
}

TEST(layer,add)
{
	region st("",12);

	st.add(bound(0,6),layer_loc(new layer(anonymous_layer({1,2,3,4,5,6},"anon 2"))));
	st.add(bound(10,40),layer_loc(new layer(anonymous_layer({1,2,3,4,5,6},"anon 3"))));
	st.add(bound(4,12),layer_loc(new layer(anonymous_layer({1,2,3,4,5,6},"anon 4"))));
	auto proj = st.projection();

	for(const std::pair<bound,layer_wloc> &p: proj)
		std::cout << p.first << ": " << name(*p.second) << std::endl;
}

TEST(layer,projection)
{
	region st("",134);
	layer_loc base(new layer(anonymous_layer({},"base")));
	layer_loc xor1(new layer(anonymous_layer({},"xor")));
	layer_loc add(new layer(anonymous_layer({},"add")));
	layer_loc zlib(new layer(anonymous_layer({},"zlib")));
	layer_loc aes(new layer(anonymous_layer({},"aes")));

	st.add(bound(0,128),base);
	st.add(bound(0,64),xor1);
	st.add(bound(45,72),add);
	st.add(bound(80,128),zlib);
	st.add(bound(102,134),aes);

	auto proj = st.projection();
	boost::icl::interval_map<offset,layer_wloc> expect;

	expect += std::make_pair(bound(0,45),layer_wloc(xor1));
	expect += std::make_pair(bound(45,72),layer_wloc(add));
	expect += std::make_pair(bound(72,80),layer_wloc(base));
	expect += std::make_pair(bound(80,102),layer_wloc(zlib));
	expect += std::make_pair(bound(102,134),layer_wloc(aes));

	std::cerr << "proj:" << std::endl;
	for(const std::pair<bound,layer_wloc> &p: proj)
		std::cerr << p.first << " => " << name(*p.second) << std::endl;
	std::cerr << "expect:" << std::endl;
	for(const std::pair<bound,layer_wloc> &p: expect)
		std::cerr << p.first << " => " << name(*p.second) << std::endl;
	ASSERT_TRUE(proj == expect);
}

TEST(region,tree)
{
	regions regs;
	region_loc r1(new region("base",128));
	region_loc r2(new region("zlib",64));
	region_loc r3(new region("aes",48));

	auto vx1 = regs.insert_node(r1);
	auto vx2 = regs.insert_node(r2);
	auto vx3 = regs.insert_node(r3);

	regs.insert_edge(bound(32,96),vx2,vx1);
	regs.insert_edge(bound(16,32),vx3,vx1);
	regs.insert_edge(bound(0,32),vx3,vx2);

	auto t = spanning_tree(regs);
	decltype(t) expect({
		make_pair(region_wloc(r2),region_wloc(r1)),
		make_pair(region_wloc(r3),region_wloc(r1))
	});

	for(auto i: t)
	{
		std::cout << i.first->name() << " -> " << i.second->name() << std::endl;
	}

	ASSERT_TRUE(t == expect);
}

TEST(region,proj)
{
	regions regs;
	region_loc r1(new region("base",128));
	region_loc r2(new region("zlib",64));
	region_loc r3(new region("aes",48));

	auto vx1 = regs.insert_node(r1);
	auto vx2 = regs.insert_node(r2);
	auto vx3 = regs.insert_node(r3);

	regs.insert_edge(bound(32,96),vx2,vx1);
	regs.insert_edge(bound(16,32),vx3,vx1);
	regs.insert_edge(bound(0,32),vx3,vx2);

	auto proj = projection(regs);
	decltype(proj) expect({
		make_pair(bound(0,16),region_wloc(r1)),
		make_pair(bound(0,48),region_wloc(r3)),
		make_pair(bound(32,64),region_wloc(r2)),
		make_pair(bound(96,128),region_wloc(r1))
	});

	for(auto i: proj)
	{
		std::cout << i.first << ": " << i.second->name() << std::endl;
	}

	ASSERT_TRUE(proj == expect);
}
