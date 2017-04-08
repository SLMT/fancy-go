
use piston_window::{Context, G2d};
use piston_window::types::Color;
use piston_window::{line, ellipse};

use stone::{Stone, StoneType};

const NUM_OF_POINTS: usize = 19;
const POINT_SPACING: f64 = 50.0;
const LINE_RADIUS: f64 = 1.0;
const LINE_LENGTH: f64 = ((NUM_OF_POINTS - 1) as f64) * POINT_SPACING;
const LINE_COLOR: Color = [0.43, 0.57, 0.76, 1.0];
const SHADOW_OFFSET: f64 = 8.0;
const SHADOW_COLOR: Color = [0.18, 0.21, 0.34, 1.0];
const STAR_POINT_RADIUS: f64 = 10.0;

pub struct Board {
    position: [f64; 2],
    stones: Vec<Stone>
}

impl Board {
    pub fn new(pos_x: f64, pos_y: f64) -> Board {
        let mut board = Board {
            position: [pos_x, pos_y],
            stones: Vec::new()
        };

        board.stones.push(Stone::new(pos_x, pos_y, StoneType::WHITE));
        board.stones.push(Stone::new(pos_x + POINT_SPACING, pos_y, StoneType::BLACK));

        board
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.draw_line_shadows(con, g);
        self.draw_lines(con, g);
        self.draw_star_points(con, g);

        for stone in &self.stones {
            stone.draw(con, g);
        }
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

    fn draw_star_points(&self, con: &Context, g: &mut G2d) {
        let points: Vec<[i32; 2]> = vec![
            [3, 3], [9, 3], [15, 3],
            [3, 9], [9, 9], [15, 9],
            [3, 15], [9, 15], [15, 15]
        ];

        for point in points {
            let point_x = self.position[0] +
                (point[0] as f64) * POINT_SPACING;
            let point_y = self.position[1] +
                (point[1] as f64) * POINT_SPACING;

            let x = point_x - STAR_POINT_RADIUS / 2.0;
            let y = point_y - STAR_POINT_RADIUS / 2.0;
            let w = STAR_POINT_RADIUS;
            let h = STAR_POINT_RADIUS;
            ellipse(LINE_COLOR, [x, y, w, h], con.transform, g);
        }
    }
}
