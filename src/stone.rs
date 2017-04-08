
use std::f64::consts::{PI};

use piston_window::{Context, G2d};
use piston_window::types::Color;
use piston_window::polygon;

const RADIUS: f64 = 20.0;
const BLACK_COLOR: Color = [0.0, 0.0, 0.0, 1.0];

pub struct Stone {
    center: [f64; 2]
}

impl Stone {
    pub fn new(pos_x: f64, pos_y: f64) -> Stone {
        Stone {
            center: [pos_x, pos_y]
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        let points = hexagon(self.center);
        polygon(BLACK_COLOR, &points, con.transform, g);
    }
}

fn hexagon(center: [f64; 2]) -> Vec<[f64; 2]> {
    let mut points: Vec<[f64; 2]> = Vec::new();
    for i in 0..6 {
        let mut point = [0.0, 0.0];
        let base: f64 = PI / 6.0 * (1 + 2 * i) as f64;
        point[0] = center[0] + RADIUS * base.cos();
        point[1] = center[1] + RADIUS * base.sin();
        points.push(point);
    }
    points
}
