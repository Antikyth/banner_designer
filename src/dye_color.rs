// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::fmt::{Display, Formatter};
use wasm_bindgen::prelude::wasm_bindgen;

/// Represents the color of a banner or a pattern applied to it.
// Note: the colors are split up into groups of 4 purely to make it easier to keep track of where
// you're reading. There is no significance to their grouping.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[wasm_bindgen]
#[repr(u8)]
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

impl Default for DyeColor {
	fn default() -> Self {
		Self::White
	}
}

impl Display for DyeColor {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.identifier())
	}
}

// Because JavaScript doesn't actually have an enum equivalent, these can't be associated functions.

/// Returns the hex color code for the given `DyeColor` as a string for use in rendering on the canvas.
#[wasm_bindgen(js_name = dyeColorToHex)]
pub fn dye_color_to_hex(color: DyeColor) -> String {
	match color {
		DyeColor::White => "#f9fffe",
		DyeColor::Orange => "#f9801d",
		DyeColor::Magenta => "#c74ebd",
		DyeColor::LightBlue => "#3ab3da",

		DyeColor::Yellow => "#fed83d",
		DyeColor::Lime => "#80c71f",
		DyeColor::Pink => "#f38baa",
		DyeColor::Gray => "#474f52",

		DyeColor::LightGray => "#9d9d97",
		DyeColor::Cyan => "#169c9c",
		DyeColor::Purple => "#8932b8",
		DyeColor::Blue => "#3c44aa",

		DyeColor::Brown => "#835432",
		DyeColor::Green => "#5e7c16",
		DyeColor::Red => "#b02e26",
		DyeColor::Black => "#1d1d21",
	}
	.to_owned()
}

/// Returns the numerical ID of the color, used to refer to the color of a [pattern] in commands.
///
/// [pattern]: Pattern
//#[wasm_bindgen(js_name = dyeColorID)] -- is this just the way they'll appear in JS anyway?
pub fn dye_color_id(color: DyeColor) -> u8 {
	match color {
		DyeColor::White => 0,
		DyeColor::Orange => 1,
		DyeColor::Magenta => 2,
		DyeColor::LightBlue => 3,

		DyeColor::Yellow => 4,
		DyeColor::Lime => 5,
		DyeColor::Pink => 6,
		DyeColor::Gray => 7,

		DyeColor::LightGray => 8,
		DyeColor::Cyan => 9,
		DyeColor::Purple => 10,
		DyeColor::Blue => 11,

		DyeColor::Brown => 12,
		DyeColor::Green => 13,
		DyeColor::Red => 14,
		DyeColor::Black => 15,
	}
}

impl DyeColor {
	/// Returns the identifier used to refer to the color of patterns in lang files.
	pub fn identifier(&self) -> &str {
		match self {
			Self::White => "white",
			Self::Orange => "orange",
			Self::Magenta => "magenta",
			Self::LightBlue => "light_blue",

			Self::Yellow => "yellow",
			Self::Lime => "lime",
			Self::Pink => "pink",
			Self::Gray => "gray",

			Self::LightGray => "light_gray",
			Self::Cyan => "cyan",
			Self::Purple => "purple",
			Self::Blue => "blue",

			Self::Brown => "brown",
			Self::Green => "green",
			Self::Red => "red",
			Self::Black => "black",
		}
	}
}
