/*
 * This file is part of Panopticon (http://panopticon.re).
 * Copyright (C) 2014 Kai Michaelis
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

#include <panopticon/disassembler.hh>

#include <panopticon/amd64/amd64.hh>

#pragma once

namespace po
{
	namespace amd64
	{
		using dis = disassembler<amd64_tag>;

		void add_generic(
			dis& main, dis const& opsize_prfix, dis const& rex_prfix, dis const& rexw_prfix,
			dis const& generic_prfx, dis const& addrsize_prfx, dis const& rep_prfx,
			dis const& imm8, dis const& imm16, dis const& imm32, dis const& imm64,
			dis const& sib,
			dis const& rm8, dis const& rm16, dis const& rm32, dis const& rm64,
			dis const& rm8_0, dis const& rm16_0, dis const& rm32_0, dis const& rm64_0,
			dis const& rm8_1, dis const& rm16_1, dis const& rm32_1, dis const& rm64_1,
			dis const& rm8_2, dis const& rm16_2, dis const& rm32_2, dis const& rm64_2,
			dis const& rm8_3, dis const& rm16_3, dis const& rm32_3, dis const& rm64_3,
			dis const& rm8_4, dis const& rm16_4, dis const& rm32_4, dis const& rm64_4,
			dis const& rm8_5, dis const& rm16_5, dis const& rm32_5, dis const& rm64_5,
			dis const& rm8_6, dis const& rm16_6, dis const& rm32_6, dis const& rm64_6,
			dis const& rm8_7, dis const& rm16_7, dis const& rm32_7, dis const& rm64_7,
			dis const& disp8, dis const& disp16, dis const& disp32, dis const& disp64);
	}
}
