
use std::f64::consts::{PI};

use piston_window::{Context, G2d};
use piston_window::types::Color;
use piston_window::{line, polygon};

use settings::{STONE_RADIUS, SHADOW_OFFSET, SHADOW_COLOR};
use settings::{BLACK_COLOR, PURPLE_COLOR, WHITE_PURPLE_COLOR, WHITE_COLOR};
use settings::{POINT_SPACING};

#[derive(Clone, Copy)]
pub enum StoneType {
    BLACK, WHITE
}

pub struct Stone {
    shadow: Hexagon,
    hexes: Vec<Hexagon>,
    animation: Option<Animation>
}

impl Stone {
    pub fn new(pos_x: f64, pos_y: f64, stone_type: StoneType) -> Stone {
        let center = [pos_x, pos_y];
        let shadow_center = [pos_x + SHADOW_OFFSET, pos_y + SHADOW_OFFSET];
        let mut hexes: Vec<Hexagon> = Vec::new();

        match stone_type {
            StoneType::BLACK => {
                hexes.push(Hexagon::new(center, STONE_RADIUS, PURPLE_COLOR));
                hexes.push(Hexagon::new(center, STONE_RADIUS - 5.0, BLACK_COLOR));
                hexes.push(Hexagon::new(center, STONE_RADIUS - 6.5, WHITE_PURPLE_COLOR));
                hexes.push(Hexagon::new(center, STONE_RADIUS - 9.0, BLACK_COLOR));
            },
            StoneType::WHITE => {
                hexes.push(Hexagon::new(center, STONE_RADIUS, PURPLE_COLOR));
                hexes.push(Hexagon::new(center, STONE_RADIUS - 2.0, BLACK_COLOR));
                hexes.push(Hexagon::new(center, STONE_RADIUS - 3.5, WHITE_PURPLE_COLOR));
                hexes.push(Hexagon::new(center, STONE_RADIUS - 7.0, BLACK_COLOR));
                hexes.push(Hexagon::new(center, STONE_RADIUS - 10.0, WHITE_COLOR));
            }
        }

        Stone {
            shadow: Hexagon::new(shadow_center, STONE_RADIUS, SHADOW_COLOR),
            hexes: hexes,
            animation: Some(Animation::new(pos_x, pos_y))
        }
    }

    pub fn draw_shadow(&self, con: &Context, g: &mut G2d) {
        match self.animation {
            Some(ref ani) => {
                // TODO: Draw animation shadows
            },
            None => {
                self.shadow.draw_filled(con, g);
            }
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        match self.animation {
            Some(ref ani) => {
                ani.draw(con, g);
            },
            None => {
                for hex in &self.hexes {
                    hex.draw_filled(con, g);
                }
            }
        }
    }

    pub fn update(&mut self, delta: f64) {
        let mut turn_off_animation = false;

        if let Some(ref mut ani) = self.animation {
            ani.update(delta);

            if ani.is_finished() {
                turn_off_animation = true;
            }
        }

        if turn_off_animation {
            self.animation = None;
        }
    }
}

struct Hexagon {
    points: Vec<[f64; 2]>,
    color: Color
}

impl Hexagon {
    fn new(center: [f64; 2], radius: f64, color: Color) -> Hexagon {
        let mut points: Vec<[f64; 2]> = Vec::new();
        for i in 0..6 {
            let mut point = [0.0, 0.0];
            let base: f64 = PI / 6.0 * (1 + 2 * i) as f64;
            point[0] = center[0] + radius * base.cos();
            point[1] = center[1] + radius * base.sin();
            points.push(point);
        }

        Hexagon {
            points: points,
            color: color
        }
    }

    fn get_lower_right(&self) -> [f64; 2] {
        self.points[0]
    }

    fn get_lower(&self) -> [f64; 2] {
        self.points[1]
    }

    fn get_lower_left(&self) -> [f64; 2] {
        self.points[2]
    }

    fn get_upper_left(&self) -> [f64; 2] {
        self.points[3]
    }

    fn get_upper(&self) -> [f64; 2] {
        self.points[4]
    }

    fn get_upper_right(&self) -> [f64; 2] {
        self.points[5]
    }

    fn draw_filled(&self, con: &Context, g: &mut G2d) {
        polygon(self.color, &self.points, con.transform, g);
    }

    fn draw_lined(&self, line_radius: f64, con: &Context, g: &mut G2d) {
        for i in 0..6 {
            let start_x = self.points[i][0];
            let start_y = self.points[i][1];
            let mut end_x = self.points[0][0];
            let mut end_y = self.points[0][1];

            if i < 5 {
                end_x = self.points[i + 1][0];
                end_y = self.points[i + 1][1];
            }

            line(self.color, line_radius, [start_x, start_y, end_x, end_y], con.transform, g);
        }
    }
}

const ANIMATION_RADIUS: f64 = POINT_SPACING * 6.0;
const AIMMING_TIME: f64 = 0.3;
const PHASE1_TIME: f64 = 1.0;
const PHASE2_START: f64 = 1.0;
const PHASE2_AIMMING_TIME: f64 = 0.3;
const PHASE2_END: f64 = 2.0;
const AIMMING_HEX_RADIUS: f64 = 0.5;
const AIMMING_LINE_RADIUS: f64 = 1.5;
const LIGHT_GREEN_COLOR: Color = [0.19, 0.98, 0.53, 1.0];
const INNER_HEX_FLASH_PERIOD: f64 = 0.1;

const PHASE2_COLOR1: Color = [0.30, 0.41, 0.59, 0.75];
const PHASE2_COLOR2: Color = [0.35, 0.60, 0.59, 0.75];
const PHASE2_LINE_COLOR: Color = [1.0, 1.0, 1.0, 0.25];

struct Animation {
    center: [f64; 2],
    elapsed_time: f64,
    finished: bool
}

impl Animation {
    fn new(pos_x: f64, pos_y: f64) -> Animation {
        Animation {
            center: [pos_x, pos_y],
            elapsed_time: 0.0,
            finished: false
        }
    }

    fn draw(&self, con: &Context, g: &mut G2d) {
        let et = self.elapsed_time;

        // Phase 1 (Green line aimming)
        if et < PHASE2_END {
            // Scale
            let scale = 1.0 - et / AIMMING_TIME;

            // Outter Hexagon
            self.draw_outter_hexagon(scale, con, g);

            // Aimming lines
            self.draw_aimming_lines(scale, con, g);

            // Inner Hexagon
            self.draw_inner_hexagon(scale, con, g);
        }

        // Phase 2 (transparent hex)
        if (et > PHASE2_START) && (et < PHASE2_END) {
            self.draw_transparent_hex(con, g);
        }
    }

    fn draw_outter_hexagon(&self, scale: f64, con: &Context, g: &mut G2d) {
        let hex_radius = POINT_SPACING * 6.0 * scale;
        if hex_radius > POINT_SPACING {
            let hex = Hexagon::new(self.center, hex_radius, LIGHT_GREEN_COLOR);
            hex.draw_lined(AIMMING_HEX_RADIUS, con, g)
        } else {
            let hex = Hexagon::new(self.center, POINT_SPACING, LIGHT_GREEN_COLOR);
            hex.draw_lined(AIMMING_HEX_RADIUS, con, g)
        }
    }

    fn draw_aimming_lines(&self, scale: f64, con: &Context, g: &mut G2d) {
        let c_x = self.center[0];
        let c_y = self.center[1];

        // Draw draw the outter part
        let aim_line_r = POINT_SPACING * 5.0;
        let mut to_center = aim_line_r * scale;
        if to_center < POINT_SPACING * 0.6 {
            to_center = POINT_SPACING * 0.6;
        }
        let lines = [
            [c_x - aim_line_r, c_y, c_x - to_center, c_y],
            [c_x + aim_line_r, c_y, c_x + to_center, c_y],
            [c_x, c_y - aim_line_r, c_x, c_y - to_center],
            [c_x, c_y + aim_line_r, c_x, c_y + to_center]
        ];
        for i in 0 .. 4 {
            line(LIGHT_GREEN_COLOR, AIMMING_LINE_RADIUS, lines[i], con.transform, g);
        }

        // Draw the center part
        let h = Hexagon::new(self.center, to_center, LIGHT_GREEN_COLOR);

        let lines = [
            // Upper left
            [h.get_upper()[0],
            h.get_upper()[1],
            (h.get_upper()[0] - h.get_upper_left()[0]) * 0.75 + h.get_upper_left()[0],
            (h.get_upper()[1] - h.get_upper_left()[1]) * 0.75 + h.get_upper_left()[1]],
            // Upper right
            [h.get_upper()[0],
            h.get_upper()[1],
            (h.get_upper()[0] - h.get_upper_right()[0]) * 0.75 + h.get_upper_right()[0],
            (h.get_upper()[1] - h.get_upper_right()[1]) * 0.75 + h.get_upper_right()[1]],
            // Lower left
            [h.get_lower()[0],
            h.get_lower()[1],
            (h.get_lower()[0] - h.get_lower_left()[0]) * 0.75 + h.get_lower_left()[0],
            (h.get_lower()[1] - h.get_lower_left()[1]) * 0.75 + h.get_lower_left()[1]],
            // Lower right
            [h.get_lower()[0],
            h.get_lower()[1],
            (h.get_lower()[0] - h.get_lower_right()[0]) * 0.75 + h.get_lower_right()[0],
            (h.get_lower()[1] - h.get_lower_right()[1]) * 0.75 + h.get_lower_right()[1]],
            // Left
            [h.get_lower_left()[0],
            (h.get_upper_left()[1] - h.get_lower_left()[1]) * 0.25 + h.get_lower_left()[1],
            h.get_lower_left()[0],
            (h.get_upper_left()[1] - h.get_lower_left()[1]) * 0.75 + h.get_lower_left()[1]],
            // Right
            [h.get_lower_right()[0],
            (h.get_upper_right()[1] - h.get_lower_right()[1]) * 0.25 + h.get_lower_right()[1],
            h.get_lower_right()[0],
            (h.get_upper_right()[1] - h.get_lower_right()[1]) * 0.75 + h.get_lower_right()[1]],
        ];

        for i in 0 .. 6 {
            line(LIGHT_GREEN_COLOR, AIMMING_LINE_RADIUS, lines[i], con.transform, g);
        }
    }

    fn draw_inner_hexagon(&self, scale: f64, con: &Context, g: &mut G2d) {
        let outter_hex_radius = ANIMATION_RADIUS * scale;
        if outter_hex_radius <= POINT_SPACING * 0.5 {
            let remain_time = self.elapsed_time / INNER_HEX_FLASH_PERIOD % 2.0;

            if remain_time > 1.0 {
                let hex = Hexagon::new(self.center, POINT_SPACING * 0.5, LIGHT_GREEN_COLOR);
                hex.draw_lined(AIMMING_LINE_RADIUS, con, g)
            }
        }
    }

    fn draw_transparent_hex(&self, con: &Context, g: &mut G2d) {
        let scale = (1.0 - (self.elapsed_time - PHASE2_START) / PHASE2_AIMMING_TIME).max(0.0);
        let min_size = POINT_SPACING * 1.5;

        // Level 1 (white thin, white thin)
        let to_center = ((ANIMATION_RADIUS * 0.8 - min_size) * scale) + min_size;
        let hex1 = Hexagon::new(self.center, to_center * 0.9, PHASE2_LINE_COLOR);
        hex1.draw_lined(AIMMING_HEX_RADIUS, con, g);
        let hex2 = Hexagon::new(self.center, to_center * 1.1, PHASE2_LINE_COLOR);
        hex2.draw_lined(AIMMING_HEX_RADIUS, con, g);

        // Level 2 ()
        let to_center = ((ANIMATION_RADIUS - min_size) * scale) + min_size;
        let hex1 = Hexagon::new(self.center, to_center, PHASE2_COLOR1);
        hex1.draw_lined(AIMMING_HEX_RADIUS * 12.0, con, g);
        let hex2 = Hexagon::new(self.center, to_center * 1.2, PHASE2_LINE_COLOR);
        hex2.draw_lined(AIMMING_HEX_RADIUS, con, g);

        // Level 3
        let to_center = ((ANIMATION_RADIUS * 1.2 - min_size) * scale) + min_size;
        let hex1 = Hexagon::new(self.center, to_center * 1.1, PHASE2_COLOR2);
        hex1.draw_lined(AIMMING_HEX_RADIUS * 12.0, con, g);
        let hex2 = Hexagon::new(self.center, to_center * 1.3, PHASE2_LINE_COLOR);
        hex2.draw_lined(AIMMING_HEX_RADIUS, con, g);
    }

    fn update(&mut self, delta: f64) {
        self.elapsed_time += delta;
        if self.elapsed_time >= PHASE2_END {
            self.finished = true;
        }
    }

    fn is_finished(&self) -> bool {
        self.finished
    }
}
