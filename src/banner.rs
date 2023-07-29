pub const MAX_PATTERNS_SURVIVAL: usize = 7;
pub const MAX_PATTERNS_COMMANDS: usize = 17;

pub struct Banner {
	pub color: DyeColor,
	pub patterns: Vec<(Pattern, DyeColor)>,
}

impl Banner {
	pub fn name(&self) -> &str {
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

pub enum PatternItemRequired {
	No,
	BedrockOnly,
	Yes,
}

impl Pattern {
	pub fn banner_pattern_item_required(&self) -> PatternItemRequired {
		use PatternItemRequired::*;

		match self {
			Self::CurlyBorder => BedrockOnly,

			Self::Bricks => BedrockOnly,

			Self::Globe => Yes,
			Self::Creeper => Yes,
			Self::Skull => Yes,
			Self::Flower => Yes,
			Self::Mojang => Yes,
			Self::Piglin => Yes,

			_ => No,
		}
	}
}
