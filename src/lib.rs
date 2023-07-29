// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use wasm_bindgen::prelude::*;

pub use dye_color::*;
pub use pattern::*;

mod dye_color;
mod pattern;

#[cfg(feature = "wee_alloc")]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// The maximum number of patterns that can be applied to a banner without the use of commands.
pub const MAX_PATTERNS_SURVIVAL: usize = 6;
/// The maximum number of patterns that can be applied to a banner, even with the use of commands.
pub const MAX_PATTERNS_COMMANDS: usize = 16;

/// A Minecraft banner design.
#[wasm_bindgen]
pub struct Banner {
	/// The base color of the banner.
	color: DyeColor,
	patterns: Vec<(Pattern, DyeColor)>,
}

#[wasm_bindgen]
impl Banner {
	/// Creates a new banner of the given color with no patterns applied.
	#[wasm_bindgen(constructor)]
	pub fn new(color: DyeColor) -> Self {
		Self {
			color,
			patterns: Default::default(),
		}
	}

	/// Returns the identifier used in lang files to translate the name of this banner.
	#[wasm_bindgen(js_name = langIdentifier)]
	pub fn lang_identifier(&self) -> String {
		format!("block.minecraft.{}", self.identifier())
	}

	#[wasm_bindgen(getter)]
	pub fn color(&self) -> DyeColor {
		self.color
	}

	#[wasm_bindgen(setter)]
	pub fn set_color(&mut self, color: DyeColor) {
		self.color = color;
	}

	#[wasm_bindgen(getter)]
	pub fn patterns(&self) -> *const (Pattern, DyeColor) {
		self.patterns.as_ptr()
	}
}

impl Banner {
	/// Returns the identifier for the banner based on its color.
	///
	/// This is used in commands to specify the base color of the banner, and in lang files to
	/// translate the name of the banner.
	fn identifier(&self) -> &str {
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
