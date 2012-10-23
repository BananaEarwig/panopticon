#include <iostream>
#include <algorithm>
#include <iterator>

#include <cppunit/extensions/HelperMacros.h>
#include <flowgraph.hh>
#include <decoder.hh>

struct test_tag {};
unsigned int ununsed = 0;

template<>
struct architecture_traits<test_tag>
{
	typedef char token_type;
};

template<>
var_ptr temporary(test_tag)
{
	return var_ptr(new variable("t" + to_string(ununsed++),16));
}

class CodeGeneratorTest : public CppUnit::TestFixture
{
	CPPUNIT_TEST_SUITE(CodeGeneratorTest);
	CPPUNIT_TEST(test);
	CPPUNIT_TEST_SUITE_END();

public:

	void setUp(void)
	{
		return;
	}

	void tearDown(void)
	{
		return;
	}

	void test(void)
	{
		typedef sem_state<test_tag>& ss;
		typedef code_generator<test_tag>& cg;

		vector<char> bytes({'A','A','B','A','C'});
		decoder<test_tag> main, sub;
		proc_ptr proc(new procedure());
		
		sub | 'B' = [](ss st)
		{
			std::cout << "#sub::BA" << endl;
			st.mnemonic(2,"BA");
			st.jump(st.address + 2);
		};

		main | 'A' | sub = [](ss st)
		{
			;
		};
		
		main | 'A' = [](ss st)
		{
			std::cout << "#main::A" << endl;
			st.mnemonic(1,"A");
			st.jump(st.address + 1);
		};

		main | 'C' = [](ss st)
		{
			std::cout << "#main::C" << endl;
			st.mnemonic(1,"C");
			st.jump(st.address + 1);
		};

		main = [](ss st)
		{
			std::cout << "#failsafe" << endl;
			st.mnemonic(1,"UNK");
			st.jump(st.address + 1);
		};


		disassemble_procedure(proc,main,bytes,0);
		flow_ptr fg(new flowgraph());
		fg->procedures.insert(proc);

	//	cout << graphviz(fg) << endl;
		return;
	}
};
