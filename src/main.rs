extern crate piston_window;
extern crate glutin;

mod game;
mod board;
mod stone;
mod settings;

use piston_window::{PistonWindow, WindowSettings, Button, PressEvent, UpdateEvent, MouseCursorEvent};
use piston_window::mouse::MouseButton;
use piston_window::clear;
use piston_window::types::Color;
use glutin::MouseCursor;

use game::Game;

const WINDOW_TITLE: &'static str = "Fancy Go";
const WINDOW_SIZE: [u32; 2] = [1000, 1000];

const BACK_COLOR: Color = [0.23, 0.25, 0.39, 1.0];

fn main() {
    // Create a window
    let mut window: PistonWindow = WindowSettings::new(WINDOW_TITLE, WINDOW_SIZE)
            .exit_on_esc(true).build().unwrap();

    // Create a game
    let mut game = Game::new([50.0, 50.0]);

    // Mouse position
    let (mut mouse_x, mut mouse_y): (f64, f64) = (0.0, 0.0);

    // Event loop
    while let Some(event) = window.next() {

        // Press Event
        if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
            // println!("Pressed at {}, {}", mouse_x, mouse_y);
            if game.is_placeable(mouse_x, mouse_y) {
                game.place_a_stone(mouse_x, mouse_y);
            }
        }

        // Mouse Moving Event
        if let Some(mouse_pos) = event.mouse_cursor_args() {
            mouse_x = mouse_pos[0];
            mouse_y = mouse_pos[1];

            if game.is_placeable(mouse_x, mouse_y) {
                window.window.window.set_cursor(MouseCursor::Hand);
            } else {
                window.window.window.set_cursor(MouseCursor::Default);
            }
        }

        // Draw all of them
        window.draw_2d(&event, |c, g| {
            clear(BACK_COLOR, g);

            game.draw(&c, g);
        });

        // Update the state of the game
        if let Some(arg) = event.update_args() {
            game.update(arg.dt);
        }
    }
}
