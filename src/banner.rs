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
