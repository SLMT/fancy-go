
use std::f64::consts::{PI};

use piston_window::{Context, G2d};
use piston_window::types::Color;
use piston_window::polygon;

const RADIUS: f64 = 21.0;
const BLACK_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
const PURPLE_COLOR: Color = [0.65, 0.56, 0.97, 0.3];
const WHITE_PURPLE_COLOR: Color = [0.88, 0.79, 0.95, 1.0];
const WHITE_COLOR: Color = [1.0, 1.0, 1.0, 1.0];

pub enum StoneType {
    BLACK, WHITE
}

pub struct Stone {
    hexes: Vec<Hexagon>
}

impl Stone {
    pub fn new(pos_x: f64, pos_y: f64, stone_type: StoneType) -> Stone {
        let center = [pos_x, pos_y];
        let mut hexes: Vec<Hexagon> = Vec::new();

        match stone_type {
            StoneType::BLACK => {
                hexes.push(Hexagon::new(center, RADIUS, PURPLE_COLOR));
                hexes.push(Hexagon::new(center, RADIUS - 5.0, BLACK_COLOR));
                hexes.push(Hexagon::new(center, RADIUS - 6.0, WHITE_PURPLE_COLOR));
                hexes.push(Hexagon::new(center, RADIUS - 8.0, BLACK_COLOR));
            },
            StoneType::WHITE => {
                hexes.push(Hexagon::new(center, RADIUS, PURPLE_COLOR));
                hexes.push(Hexagon::new(center, RADIUS - 2.0, BLACK_COLOR));
                hexes.push(Hexagon::new(center, RADIUS - 3.0, WHITE_PURPLE_COLOR));
                hexes.push(Hexagon::new(center, RADIUS - 6.0, BLACK_COLOR));
                hexes.push(Hexagon::new(center, RADIUS - 9.0, WHITE_COLOR));
            }
        }

        Stone {
            hexes: hexes
        }
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
