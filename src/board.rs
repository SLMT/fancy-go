
use piston_window::{Context, G2d};
use piston_window::{line, ellipse};

use stone::{Stone, StoneType};
use settings::{
    NUM_OF_POINTS, POINT_SPACING, LINE_RADIUS, LINE_LENGTH,
    STAR_POINT_RADIUS, SHADOW_OFFSET, PALCEABLE_RADIUS,
    LINE_COLOR, SHADOW_COLOR, BORDER1_COLOR, BORDER2_COLOR, BORDER_RADIUS
};

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
            for _ in 0 .. NUM_OF_POINTS {
                board.stones[sx].push(None);
            }
        }

        // XXX: Debug
        // board.stones[0][0] = Some(Stone::new(pos_x, pos_y, StoneType::WHITE));
        // board.stones[0][1] = Some(Stone::new(pos_x + POINT_SPACING, pos_y, StoneType::BLACK));

        board
    }

    pub fn place_a_stone(&mut self, pos_x: f64, pos_y: f64, stone_type: StoneType) {
        let point = find_the_closet_point_2d(pos_x - self.position[0], pos_y - self.position[1]);
        if let Some((px, py)) = point {
            if self.stones[px][py].is_none() {
                let (pos_x, pos_y) = to_2d_coordinates(px, py);
                self.stones[px][py] = Some(Stone::new(
                    pos_x + self.position[0], pos_y + self.position[1], stone_type));
            }
        }
    }

    pub fn is_placeable(&self, pos_x: f64, pos_y: f64) -> bool {
        let point = find_the_closet_point_2d(pos_x - self.position[0], pos_y - self.position[1]);
        if let Some((px, py)) = point {
            self.stones[px][py].is_none()
        } else {
            false
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.draw_line_shadows(con, g);

        for sx in 0 .. NUM_OF_POINTS {
            for sy in 0 .. NUM_OF_POINTS {
                if let Some(ref stone) = self.stones[sx][sy] {
                    stone.draw_shadow(con, g);
                }
            }
        }

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

fn to_2d_coordinates(point_x: usize, point_y: usize) -> (f64, f64) {
    (point_x as f64 * POINT_SPACING, point_y as f64 * POINT_SPACING)
}

fn find_the_closet_point_2d(relative_pos_x: f64, relative_pos_y: f64) -> Option<(usize, usize)> {
    let point_pos_x = find_the_closet_point(relative_pos_x);
    if let Some(px) = point_pos_x {
        let point_pos_y = find_the_closet_point(relative_pos_y);
        if let Some(py) = point_pos_y {
            if px < NUM_OF_POINTS && py < NUM_OF_POINTS {
                Some((px, py))
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

fn find_the_closet_point(relative_pos: f64) -> Option<usize> {
    // Check the lower bound
    if relative_pos < - PALCEABLE_RADIUS {
        None
    } else if relative_pos < PALCEABLE_RADIUS {
        Some(0)
    } else {
        let quotient = (relative_pos / POINT_SPACING).floor() as usize;
        let remainder = relative_pos % POINT_SPACING;

        if remainder <= PALCEABLE_RADIUS {
            Some(quotient)
        } else if remainder >= POINT_SPACING - PALCEABLE_RADIUS {
            Some(quotient + 1)
        } else {
            None
        }
    }
}
