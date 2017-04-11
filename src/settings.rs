
use piston_window::types::Color;

// Board
pub const NUM_OF_POINTS: usize = 19;
pub const POINT_SPACING: f64 = 50.0;
pub const LINE_RADIUS: f64 = 1.0;
pub const LINE_LENGTH: f64 = ((NUM_OF_POINTS - 1) as f64) * POINT_SPACING;
pub const STAR_POINT_RADIUS: f64 = 10.0;
pub const PALCEABLE_RADIUS: f64 = 10.0;

// Shadow
pub const SHADOW_OFFSET: f64 = 8.0;

// Stone
pub const STONE_RADIUS: f64 = 25.0;

// Colors
pub const LINE_COLOR: Color = [0.43, 0.57, 0.76, 1.0];
pub const SHADOW_COLOR: Color = [0.18, 0.21, 0.34, 1.0];
pub const BORDER1_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
pub const BORDER2_COLOR: Color = [0.54, 0.54, 0.81, 1.0];
pub const BORDER_RADIUS: f64 = 2.0;
pub const BLACK_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
pub const PURPLE_COLOR: Color = [0.65, 0.56, 0.97, 0.3];
pub const WHITE_PURPLE_COLOR: Color = [0.88, 0.79, 0.95, 1.0];
pub const WHITE_COLOR: Color = [1.0, 1.0, 1.0, 1.0];
