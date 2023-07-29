// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::pattern::Pattern;

/// Represents the color of a banner or a pattern applied to it.
// Note: the colors are split up into groups of 4 purely to make it easier to keep track of where
// you're reading. There is no significance to their grouping.
pub enum DyeColor {
	White,
	Orange,
	Magenta,
	LightBlue,

	Yellow,
	Lime,
	Pink,
	Gray,

	LightGray,
	Cyan,
	Purple,
	Blue,

	Brown,
	Green,
	Red,
	Black,
}

impl DyeColor {
	pub fn color(&self) -> u32 {
		match self {
			Self::White => 0xf9fffe,
			Self::Orange => 0xf9801d,
			Self::Magenta => 0xc74ebd,
			Self::LightBlue => 0x3ab3da,

			Self::Yellow => 0xfed83d,
			Self::Lime => 0x80c71f,
			Self::Pink => 0xf38baa,
			Self::Gray => 0x474f52,

			Self::LightGray => 0x9d9d97,
			Self::Cyan => 0x169c9c,
			Self::Purple => 0x8932b8,
			Self::Blue => 0x3c44aa,

			Self::Brown => 0x835432,
			Self::Green => 0x5e7c16,
			Self::Red => 0xb02e26,
			Self::Black => 0x1d1d21,
		}
	}

	/// Returns the numerical ID of the color, used to refer to the color of a [pattern] in commands.
	///
	/// [pattern]: Pattern
	pub fn id(&self) -> u8 {
		match self {
			Self::White => 0,
			Self::Orange => 1,
			Self::Magenta => 2,
			Self::LightBlue => 3,

			Self::Yellow => 4,
			Self::Lime => 5,
			Self::Pink => 6,
			Self::Gray => 7,

			Self::LightGray => 8,
			Self::Cyan => 9,
			Self::Purple => 10,
			Self::Blue => 11,

			Self::Brown => 12,
			Self::Green => 13,
			Self::Red => 14,
			Self::Black => 15,
		}
	}
}
