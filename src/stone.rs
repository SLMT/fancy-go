
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
                hexes.push(Hexagon::new(center, STONE_RADIUS - 6.0, WHITE_PURPLE_COLOR));
                hexes.push(Hexagon::new(center, STONE_RADIUS - 8.0, BLACK_COLOR));
            },
            StoneType::WHITE => {
                hexes.push(Hexagon::new(center, STONE_RADIUS, PURPLE_COLOR));
                hexes.push(Hexagon::new(center, STONE_RADIUS - 2.0, BLACK_COLOR));
                hexes.push(Hexagon::new(center, STONE_RADIUS - 3.0, WHITE_PURPLE_COLOR));
                hexes.push(Hexagon::new(center, STONE_RADIUS - 6.0, BLACK_COLOR));
                hexes.push(Hexagon::new(center, STONE_RADIUS - 9.0, WHITE_COLOR));
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

const ANIMATION_RADIUS: f64 = 300.0;
const HEX_AIMMING_TIME: f64 = 0.3;
const HEX_SHOWING_TIME: f64 = 2.0;
const ANIMATION_TIME: f64 = 2.0;
const LIGHT_GREEN_COLOR: Color = [0.19, 0.98, 0.53, 1.0];

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

        // Draw aiming hex
        if et < HEX_SHOWING_TIME {
            let hex_radius = ANIMATION_RADIUS * (1.0 - et / HEX_AIMMING_TIME);
            if hex_radius > POINT_SPACING {
                let hex = Hexagon::new(self.center, hex_radius, LIGHT_GREEN_COLOR);
                hex.draw_lined(1.0, con, g)
            } else {
                let hex = Hexagon::new(self.center, POINT_SPACING, LIGHT_GREEN_COLOR);
                hex.draw_lined(1.0, con, g)
            }
        }
    }

    fn update(&mut self, delta: f64) {
        self.elapsed_time += delta;
        if self.elapsed_time >= ANIMATION_TIME {
            self.finished = true;
        }
    }

    fn is_finished(&self) -> bool {
        self.finished
    }
}
