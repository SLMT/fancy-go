
use piston_window::{Context, G2d};

use board::Board;
use stone::StoneType;

pub struct Game {
    board: Board,
    next_stone_type: StoneType
}

impl Game {
    pub fn new(board_pos: [f64; 2]) -> Game {
        Game {
            board: Board::new(board_pos[0], board_pos[1]),
            next_stone_type: StoneType::BLACK
        }
    }

    pub fn place_a_stone(&mut self, pos_x: f64, pos_y: f64) {
        let next_type = self.next_stone_type;
        self.next_stone_type = match next_type {
            StoneType::BLACK => StoneType::WHITE,
            StoneType::WHITE => StoneType::BLACK
        };
        self.board.place_a_stone(pos_x, pos_y, next_type);
    }

    pub fn is_placeable(&self, pos_x: f64, pos_y: f64) -> bool {
        self.board.is_placeable(pos_x, pos_y)
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.board.draw(con, g);
    }
}
