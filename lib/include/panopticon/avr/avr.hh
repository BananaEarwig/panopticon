#include <panopticon/program.hh>

#pragma once

namespace po
{
	// architecture_traits
	struct avr_tag {};

	template<>
	struct architecture_traits<avr_tag>
	{
		using token_type = uint16_t;
		using state_type = std::nullptr_t;
	};

	template<>
	lvalue temporary(avr_tag);

	template<>
	const std::vector<std::string> &registers(avr_tag);

	template<>
	uint8_t width(std::string n, avr_tag);

	namespace avr
	{
		typedef sem_state<avr_tag> sm;
		typedef std::function<void(sm &)> sem_action;
		typedef code_generator<avr_tag> cg;

		boost::optional<prog_loc> disassemble(boost::optional<prog_loc>, po::slab, const po::ref&);
	}
}
