// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::dye_color::DyeColor;
use std::fmt::{Display, Formatter};
use wasm_bindgen::prelude::wasm_bindgen;

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
#[wasm_bindgen(js_name = patternID)]
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
#[wasm_bindgen(js_name = patternLangIdentifier)]
pub fn pattern_lang_identifier(pattern: Pattern, color: DyeColor) -> String {
	format!(
		"block.minecraft.banner.{}.{}",
		pattern.identifier(),
		color.identifier()
	)
}

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
pub enum BannerPatternItem {
	/// No banner pattern item is required to use this pattern in a loom.
	None,

	/// A banner pattern item is required to use this pattern in a loom only on Bedrock Edition.
	BedrockOnly { id: &'static str },
	/// A banner pattern item is required to use this pattern in a loom on all editions.
	AllEditions { id: &'static str },
}

impl Pattern {
	/// Returns the banner pattern required, if any, to select this pattern in a loom.
	///
	/// See [`BannerPatternItem`] for more information.
	pub fn banner_pattern_item(&self) -> BannerPatternItem {
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

/// The banner pattern item required, if any, to select a particular pattern in a loom.
#[wasm_bindgen(js_name = BannerPatternItem)]
pub struct JsBannerPatternItem {
	/// The identifier of the banner pattern item that is required.
	identifier: Option<&'static str>,

	/// Whether the banner pattern item is required on all editions, or just Bedrock Edition.
	///
	/// `true` means the banner pattern item is required on both Java Edition and Bedrock Edition.
	///
	/// `false` means the banner pattern item is only required on Bedrock Edition.
	all_editions: bool,
}

#[wasm_bindgen]
impl JsBannerPatternItem {
	#[wasm_bindgen(getter)]
	pub fn identifier(&self) -> Option<String> {
		self.identifier.map(|string| string.to_owned())
	}

	#[wasm_bindgen(getter = allEditions)]
	pub fn all_editions(&self) -> bool {
		self.all_editions
	}
}

impl From<BannerPatternItem> for JsBannerPatternItem {
	fn from(banner_pattern_item: BannerPatternItem) -> JsBannerPatternItem {
		match banner_pattern_item {
			BannerPatternItem::None => JsBannerPatternItem {
				identifier: None,
				all_editions: true,
			},

			BannerPatternItem::BedrockOnly { id: identifier } => JsBannerPatternItem {
				identifier: Some(identifier),
				all_editions: false,
			},
			BannerPatternItem::AllEditions { id: identifier } => JsBannerPatternItem {
				identifier: Some(identifier),
				all_editions: true,
			},
		}
	}
}

/// Returns the banner pattern required, if any, to select this pattern in a loom.
#[wasm_bindgen(js_name = patternBannerPatternItem)]
pub fn js_banner_pattern_item(pattern: Pattern) -> JsBannerPatternItem {
	pattern.banner_pattern_item().into()
}
