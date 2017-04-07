extern crate piston_window;

use piston_window::{PistonWindow, WindowSettings, Button, PressEvent, UpdateEvent};
use piston_window::clear;
use piston_window::types::Color;

const WINDOW_TITLE: &'static str = "Fancy Go";
const WINDOW_SIZE: [u32; 2] = [1280, 720];

const BACK_COLOR: Color = [0.204, 0.286, 0.369, 1.0];

fn main() {
    // Create a window
    let mut window: PistonWindow = WindowSettings::new(WINDOW_TITLE, WINDOW_SIZE)
            .exit_on_esc(true).build().unwrap();

    // Event loop
    while let Some(event) = window.next() {

        // Catch the events of the keyboard
        if let Some(Button::Keyboard(key)) = event.press_args() {
            // TODO
        }

        // Draw all of them
        window.draw_2d(&event, |c, g| {
            clear(BACK_COLOR, g);
            // TODO
        });

        // Update the state of the game
        event.update(|arg| {
            // TODO
        });
    }
}
