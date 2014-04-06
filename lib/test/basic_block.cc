#include <gtest/gtest.h>

#include <panopticon/basic_block.hh>

using namespace po;

TEST(basic_block,marshal)
{
	mnemonic mn1(bound(0,10),"op1","{8:-:eax} nog",{constant(1),variable("a",3)},{
		instr(instr::And,variable("a",2),{constant(1),constant(2)}),
		instr(instr::Add,variable("a",1),{constant(4),constant(2)}),
		instr(instr::Assign,variable("a",3),{variable("a",2)})});
	mnemonic mn2(bound(10,13),"op2","nig",{constant(1),variable("b",3,5)},{
		instr(instr::And,variable("b",2,6),{constant(1),constant(2)}),
		instr(instr::Add,variable("b",1,7),{constant(4),constant(2)}),
		instr(instr::Assign,variable("a",3),{variable("a",2)})});
	mnemonic mn3(bound(13,20),"op3","{8:-:eax} {9::nol}",{constant(1),variable("c",3)},{
		instr(instr::Assign,variable("c",3,5),{constant(66)})});
	uuid uu;

	bblock_loc bb1(uu,new basic_block({mn1,mn2,mn3}));
	rdf::storage store;

	save_point(store);
	ASSERT_GT(store.count(),0);

	std::unique_ptr<basic_block> bb2(unmarshal<basic_block>(uu,store));

	ASSERT_TRUE(*bb2 == *bb1);
}
