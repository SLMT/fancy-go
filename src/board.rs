
use piston_window::{Context, G2d};
use piston_window::types::Color;
use piston_window::line;

const NUM_OF_POINTS: usize = 19;
const POINT_SPACING: f64 = 50.0;
const LINE_RADIUS: f64 = 1.0;
const LINE_LENGTH: f64 = ((NUM_OF_POINTS - 1) as f64) * POINT_SPACING;
const LINE_COLOR: Color = [0.43, 0.57, 0.76, 1.0];
const SHADOW_OFFSET: f64 = 5.0;
const SHADOW_COLOR: Color = [0.18, 0.21, 0.34, 1.0];

pub struct Board {
    position: [f64; 2]
}

impl Board {
    pub fn new(pos_x: f64, pos_y: f64) -> Board {
        Board {
            position: [pos_x, pos_y]
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.draw_line_shadows(con, g);
        self.draw_lines(con, g);
    }

    fn draw_lines(&self, con: &Context, g: &mut G2d) {
        let base_x = self.position[0];
        let base_y = self.position[1];

        // Draw vertical lines
        for i in 0 .. NUM_OF_POINTS {
            let start_x = base_x + (i as f64) * POINT_SPACING;
            let start_y = base_y;

            line(LINE_COLOR, LINE_RADIUS, [start_x, start_y, start_x, start_y + LINE_LENGTH], con.transform, g);
        }

        // Draw horizontal lines
        for i in 0 .. NUM_OF_POINTS {
            let start_x = base_x;
            let start_y = base_y + (i as f64) * POINT_SPACING;

            line(LINE_COLOR, LINE_RADIUS, [start_x, start_y, start_x + LINE_LENGTH, start_y], con.transform, g);
        }
    }

    fn draw_line_shadows(&self, con: &Context, g: &mut G2d) {
        let base_x = self.position[0] + SHADOW_OFFSET;
        let base_y = self.position[1] + SHADOW_OFFSET;

        // Draw vertical lines
        for i in 0 .. NUM_OF_POINTS {
            let start_x = base_x + (i as f64) * POINT_SPACING;
            let start_y = base_y;

            line(SHADOW_COLOR, LINE_RADIUS, [start_x, start_y, start_x, start_y + LINE_LENGTH], con.transform, g);
        }

        // Draw horizontal lines
        for i in 0 .. NUM_OF_POINTS {
            let start_x = base_x;
            let start_y = base_y + (i as f64) * POINT_SPACING;

            line(SHADOW_COLOR, LINE_RADIUS, [start_x, start_y, start_x + LINE_LENGTH, start_y], con.transform, g);
        }
    }
}
