
use std::f64::consts::{PI};

use piston_window::{Context, G2d};
use piston_window::types::Color;
use piston_window::polygon;

use settings::{STONE_RADIUS, SHADOW_OFFSET, SHADOW_COLOR};
use settings::{BLACK_COLOR, PURPLE_COLOR, WHITE_PURPLE_COLOR, WHITE_COLOR};

pub enum StoneType {
    BLACK, WHITE
}

pub struct Stone {
    shadow: Hexagon,
    hexes: Vec<Hexagon>
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
            hexes: hexes
        }
    }

    pub fn draw_shadow(&self, con: &Context, g: &mut G2d) {
        self.shadow.draw(con, g);
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for hex in &self.hexes {
            hex.draw(con, g);
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

    fn draw(&self, con: &Context, g: &mut G2d) {
        polygon(self.color, &self.points, con.transform, g);
    }
}
