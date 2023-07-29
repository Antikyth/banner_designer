// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/// A pattern that can be placed on a banner.
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

impl Pattern {
	/// Returns the name of the pattern, by which its texture can be located.
	pub fn name(&self) -> &str {
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

	/// Returns the pattern ID used to refer to the pattern in commands.
	pub fn id(&self) -> &str {
		match self {
			Self::Base => "b",

			Self::SquareBottomLeft => "bl",
			Self::SquareBottomRight => "br",

			Self::SquareTopLeft => "tl",
			Self::SquareTopRight => "tr",

			Self::StripeBottom => "bs",
			Self::StripeTop => "ts",

			Self::StripeLeft => "ls",
			Self::StripeRight => "rs",

			Self::StripeCenter => "cs",
			Self::StripeMiddle => "ms",

			Self::StripeDownright => "drs",
			Self::StripeDownleft => "dls",

			Self::SmallStripes => "ss",

			Self::Cross => "cr",
			Self::StraightCross => "sc",

			Self::TriangleBottom => "bt",
			Self::TriangleTop => "tt",

			Self::TrianglesBottom => "bts",
			Self::TrianglesTop => "tts",

			Self::DiagonalLeft => "ld",
			Self::DiagonalRight => "rd",

			Self::DiagonalUpLeft => "lud",
			Self::DiagonalUpRight => "rud",

			Self::Circle => "mc",
			Self::Rhombus => "mr",

			Self::HalfVertical => "vh",
			Self::HalfHorizontal => "hh",

			Self::HalfVerticalRight => "vhr",
			Self::HalfHorizontalBottom => "hhb",

			Self::Border => "bo",
			Self::CurlyBorder => "cbo",

			Self::Gradient => "gra",
			Self::GradientUp => "gru",

			Self::Bricks => "bri",

			Self::Globe => "glb",
			Self::Creeper => "cre",
			Self::Skull => "sku",
			Self::Flower => "flo",
			Self::Mojang => "moj",
			Self::Piglin => "pig",
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
