// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use dye_color::DyeColor;
use pattern::Pattern;
use wasm_bindgen::prelude::*;

mod dye_color;
mod pattern;

#[cfg(feature = "wee_alloc")]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// The maximum number of patterns that can be applied to a banner without the use of commands.
pub const MAX_PATTERNS_SURVIVAL: usize = 6;
/// The maximum number of patterns that can be applied to a banner, even with the use of commands.
pub const MAX_PATTERNS_COMMANDS: usize = 16;

/// A Minecraft banner design.
pub struct Banner {
	/// The base color of the banner.
	pub color: DyeColor,
	patterns: [Option<(Pattern, DyeColor)>; MAX_PATTERNS_COMMANDS],
}

pub struct PatternsIter<'a> {
	iter: core::slice::Iter<'a, Option<(Pattern, DyeColor)>>,
}

impl<'a> Iterator for PatternsIter<'a> {
	type Item = &'a (Pattern, DyeColor);

	fn next(&mut self) -> Option<Self::Item> {
		self.iter.next().map_or(None, |item| item.as_ref())
	}
}

impl Banner {
	/// Creates a new banner of the given color with no patterns applied.
	pub fn new(color: DyeColor) -> Self {
		Self {
			color,
			patterns: Default::default(),
		}
	}

	/// Returns a borrowing iterator over the patterns applied to the banner.
	pub fn patterns(&self) -> PatternsIter {
		PatternsIter {
			iter: self.patterns.iter(),
		}
	}

	/// Returns the identifier for the banner based on its color.
	///
	/// This is used in commands to specify the base color of the banner.
	pub fn identifier(&self) -> &str {
		match self.color {
			DyeColor::White => "white_banner",
			DyeColor::Orange => "orange_banner",
			DyeColor::Magenta => "magenta_banner",
			DyeColor::LightBlue => "light_blue_banner",

			DyeColor::Yellow => "yellow_banner",
			DyeColor::Lime => "lime_banner",
			DyeColor::Pink => "pink_banner",
			DyeColor::Gray => "gray_banner",

			DyeColor::LightGray => "light_gray_banner",
			DyeColor::Cyan => "cyan_banner",
			DyeColor::Purple => "purple_banner",
			DyeColor::Blue => "blue_banner",

			DyeColor::Brown => "brown_banner",
			DyeColor::Green => "green_banner",
			DyeColor::Red => "red_banner",
			DyeColor::Black => "black_banner",
		}
	}
}
