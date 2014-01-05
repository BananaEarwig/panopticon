#include <iostream>

#include <gtest/gtest.h>
#include <panopticon/region.hh>

using namespace po;
using namespace std;

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
