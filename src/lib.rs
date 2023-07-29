// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::fmt::{Display, Formatter};
use wasm_bindgen::prelude::*;

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
	pub fn lang_identifier(&self) -> String {
		format!("block.minecraft.{}", self.identifier())
	}

	pub fn color(&self) -> DyeColor {
		self.color
	}

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

/// A pattern that can be placed on a banner.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[wasm_bindgen]
#[repr(u8)]
pub enum Pattern {
	Base,

	// SQUARES {{{
	SquareBottomLeft,
	SquareBottomRight,

	SquareTopLeft,
	SquareTopRight,
	// }}}

	// STRIPES {{{
	StripeBottom,
	StripeTop,

	StripeLeft,
	StripeRight,

	StripeCenter,
	StripeMiddle,

	StripeDownright,
	StripeDownleft,
	// }}}

	// Small vertical stripes
	SmallStripes,

	// CROSSES {{{
	Cross,
	StraightCross,
	// }}}

	// TRIANGLE {{{
	TriangleBottom,
	TriangleTop,
	// }}}

	// TRIANGLES {{{
	TrianglesBottom,
	TrianglesTop,
	// }}}

	// DIAGONALS {{{
	DiagonalLeft,
	DiagonalRight,

	DiagonalUpLeft,
	DiagonalUpRight,
	// }}}

	// SHAPES {{{
	Circle,
	Rhombus,
	// }}}

	// HALVES {{{
	HalfVertical,
	HalfHorizontal,

	HalfVerticalRight,
	HalfHorizontalBottom,
	// }}}

	// BORDERS {{{
	Border,
	CurlyBorder,
	// }}}

	// GRADIENTS {{{
	Gradient,
	GradientUp,
	// }}}

	// Bricks
	Bricks,

	// ICONS {{{
	Globe,
	Creeper,
	Skull,
	Flower,
	Mojang,
	Piglin,
	// }}}
}

impl Display for Pattern {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.identifier())
	}
}

// Pattern impls {{{
// Because JavaScript doesn't actually have an enum equivalent, these can't be associated functions.
/// Returns the pattern ID used to refer to the pattern in commands.
#[wasm_bindgen]
pub fn pattern_id(pattern: Pattern) -> String {
	match pattern {
		Pattern::Base => "b",

		Pattern::SquareBottomLeft => "bl",
		Pattern::SquareBottomRight => "br",

		Pattern::SquareTopLeft => "tl",
		Pattern::SquareTopRight => "tr",

		Pattern::StripeBottom => "bs",
		Pattern::StripeTop => "ts",

		Pattern::StripeLeft => "ls",
		Pattern::StripeRight => "rs",

		Pattern::StripeCenter => "cs",
		Pattern::StripeMiddle => "ms",

		Pattern::StripeDownright => "drs",
		Pattern::StripeDownleft => "dls",

		Pattern::SmallStripes => "ss",

		Pattern::Cross => "cr",
		Pattern::StraightCross => "sc",

		Pattern::TriangleBottom => "bt",
		Pattern::TriangleTop => "tt",

		Pattern::TrianglesBottom => "bts",
		Pattern::TrianglesTop => "tts",

		Pattern::DiagonalLeft => "ld",
		Pattern::DiagonalRight => "rd",

		Pattern::DiagonalUpLeft => "lud",
		Pattern::DiagonalUpRight => "rud",

		Pattern::Circle => "mc",
		Pattern::Rhombus => "mr",

		Pattern::HalfVertical => "vh",
		Pattern::HalfHorizontal => "hh",

		Pattern::HalfVerticalRight => "vhr",
		Pattern::HalfHorizontalBottom => "hhb",

		Pattern::Border => "bo",
		Pattern::CurlyBorder => "cbo",

		Pattern::Gradient => "gra",
		Pattern::GradientUp => "gru",

		Pattern::Bricks => "bri",

		Pattern::Globe => "glb",
		Pattern::Creeper => "cre",
		Pattern::Skull => "sku",
		Pattern::Flower => "flo",
		Pattern::Mojang => "moj",
		Pattern::Piglin => "pig",
	}
	.to_owned()
}

/// Returns the identifier used in lang files to translate the name of this pattern of the given color.
#[wasm_bindgen]
pub fn pattern_lang_identifier(pattern: Pattern, color: DyeColor) -> String {
	format!(
		"block.minecraft.banner.{}.{}",
		pattern.identifier(),
		color.identifier()
	)
}
// }}}

impl Pattern {
	/// Returns the name of the pattern, by which its texture can be located.
	pub fn identifier(&self) -> &str {
		match self {
			Self::Base => "base",

			Self::SquareBottomLeft => "square_bottom_left",
			Self::SquareBottomRight => "square_bottom_right",

			Self::SquareTopLeft => "square_top_left",
			Self::SquareTopRight => "square_top_right",

			Self::StripeBottom => "stripe_bottom",
			Self::StripeTop => "stripe_top",

			Self::StripeLeft => "stripe_left",
			Self::StripeRight => "stripe_right",

			Self::StripeCenter => "stripe_center",
			Self::StripeMiddle => "stripe_middle",

			Self::StripeDownleft => "stripe_downleft",
			Self::StripeDownright => "stripe_downright",

			Self::SmallStripes => "small_stripes",

			Self::Cross => "cross",
			Self::StraightCross => "straight_cross",

			Self::TriangleBottom => "triangle_bottom",
			Self::TriangleTop => "triangle_top",

			Self::TrianglesBottom => "triangles_bottom",
			Self::TrianglesTop => "triangles_top",

			Self::DiagonalLeft => "diagonal_left",
			Self::DiagonalRight => "diagonal_right",

			Self::DiagonalUpLeft => "diagonal_up_left",
			Self::DiagonalUpRight => "diagonal_up_right",

			Self::Circle => "circle",
			Self::Rhombus => "rhombus",

			Self::HalfVertical => "half_vertical_bottom",
			Self::HalfHorizontal => "half_horizontal",

			Self::HalfVerticalRight => "half_vertical_right",
			Self::HalfHorizontalBottom => "half_horizontal_bottom",

			Self::Border => "border",
			Self::CurlyBorder => "curly_border",

			Self::Gradient => "gradient",
			Self::GradientUp => "gradient_up",

			Self::Bricks => "bricks",

			Self::Globe => "globe",
			Self::Creeper => "creeper",
			Self::Skull => "skull",
			Self::Flower => "flower",
			Self::Mojang => "mojang",
			Self::Piglin => "piglin",
		}
	}
}

/// The banner pattern item required, if any, to select a particular pattern in a loom.
pub enum BannerPatternItem<'id> {
	/// No banner pattern item is required to use this pattern in a loom.
	None,

	/// A banner pattern item is required to use this pattern in a loom only on Bedrock Edition.
	BedrockOnly { id: &'id str },
	/// A banner pattern item is required to use this pattern in a loom on all editions.
	AllEditions { id: &'id str },
}

impl Pattern {
	/// Returns the banner pattern required, if any, to select this pattern in a loom.
	///
	/// See [`BannerPatternItem`] for more information.
	pub fn banner_pattern_item_required(&self) -> BannerPatternItem {
		use BannerPatternItem::*;

		match self {
			// BEDROCK ONLY {{{
			Self::CurlyBorder => BedrockOnly {
				id: "bordure_indented_banner_pattern",
			},

			Self::Bricks => BedrockOnly {
				id: "field_masoned_banner_pattern",
			},
			// }}}

			// ALL EDITIONS {{{
			Self::Globe => AllEditions {
				id: "globe_banner_pattern",
			},
			Self::Creeper => AllEditions {
				id: "creeper_banner_pattern",
			},
			Self::Skull => AllEditions {
				id: "skull_banner_pattern",
			},
			Self::Flower => AllEditions {
				id: "flower_banner_pattern",
			},
			Self::Mojang => AllEditions {
				id: "mojang_banner_pattern",
			},
			Self::Piglin => AllEditions {
				id: "piglin_banner_pattern",
			},
			// }}}
			_ => None,
		}
	}
}

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

// Dye color impls {{{
// Because JavaScript doesn't actually have an enum equivalent, these can't be associated functions.
#[wasm_bindgen]
pub fn dye_color_color(color: DyeColor) -> u32 {
	match color {
		DyeColor::White => 0x00_f9fffe,
		DyeColor::Orange => 0x00_f9801d,
		DyeColor::Magenta => 0x00_c74ebd,
		DyeColor::LightBlue => 0x00_3ab3da,

		DyeColor::Yellow => 0x00_fed83d,
		DyeColor::Lime => 0x00_80c71f,
		DyeColor::Pink => 0x00_f38baa,
		DyeColor::Gray => 0x00_474f52,

		DyeColor::LightGray => 0x00_9d9d97,
		DyeColor::Cyan => 0x00_169c9c,
		DyeColor::Purple => 0x00_8932b8,
		DyeColor::Blue => 0x00_3c44aa,

		DyeColor::Brown => 0x00_835432,
		DyeColor::Green => 0x00_5e7c16,
		DyeColor::Red => 0x00_b02e26,
		DyeColor::Black => 0x00_1d1d21,
	}
}

/// Returns the numerical ID of the color, used to refer to the color of a [pattern] in commands.
///
/// [pattern]: Pattern
#[wasm_bindgen]
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
// }}}

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
