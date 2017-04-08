
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

const BORDER1_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
const BORDER2_COLOR: Color = [0.54, 0.54, 0.81, 1.0];
const BORDER_RADIUS: f64 = 2.0;

pub struct Board {
    position: [f64; 2],
    stones: Vec<Vec<Option<Stone>>>
}

impl Board {
    pub fn new(pos_x: f64, pos_y: f64) -> Board {
        let mut board = Board {
            position: [pos_x, pos_y],
            stones: Vec::new()
        };

        // Create a (NUM_OF_POINTS x NUM_OF_POINTS) array
        for sx in 0 .. NUM_OF_POINTS {
            board.stones.push(Vec::new());
            for sy in 0 .. NUM_OF_POINTS {
                board.stones[sx].push(None);
            }
        }

        // XXX: Debug
        board.stones[0][0] = Some(Stone::new(pos_x, pos_y, StoneType::WHITE));
        board.stones[0][1] = Some(Stone::new(pos_x + POINT_SPACING, pos_y, StoneType::BLACK));

        board
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.draw_line_shadows(con, g);
        self.draw_lines(con, g);
        self.draw_boarders(con, g);
        self.draw_star_points(con, g);

        for sx in 0 .. NUM_OF_POINTS {
            for sy in 0 .. NUM_OF_POINTS {
                if let Some(ref stone) = self.stones[sx][sy] {
                    stone.draw(con, g);
                }
            }
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

    fn draw_boarders(&self, con: &Context, g: &mut G2d) {
        // Four conner points
        let left_upper = [self.position[0] - 2.0, self.position[1] - 2.0];
        let left_lower = [self.position[0] - 2.0, self.position[1] + LINE_LENGTH + 2.0];
        let right_upper = [self.position[0] + LINE_LENGTH + 2.0, self.position[1] - 2.0];
        let right_lower = [self.position[0] + LINE_LENGTH + 2.0, self.position[1] + LINE_LENGTH + 2.0];

        // Four border lines
        for (radius, color) in vec![(BORDER_RADIUS + 2.0, BORDER1_COLOR), (BORDER_RADIUS, BORDER2_COLOR)] {
            let lines: Vec<[f64; 4]> = vec![
                [left_upper[0], left_upper[1] - radius, left_lower[0], left_lower[1] + radius], // left
                [left_upper[0] - radius, left_upper[1], right_upper[0] + radius, right_upper[1]], // upper
                [right_upper[0], right_upper[1] - radius, right_lower[0], right_lower[1] + radius], // right
                [left_lower[0] - radius, left_lower[1], right_lower[0] + radius, right_lower[1]]  // lower
            ];

            // Draw them
            for l in lines {
                line(color, radius, l.clone(), con.transform, g);
            }
        }
    }
}
