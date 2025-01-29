extern crate rand;
extern crate piston_window;

mod draw;
mod game;
mod player;
mod ball;

use std::collections::HashSet;

use piston_window::*;
use piston_window::types::Color;

use crate::game::Game;
use crate::draw::to_coord_u32;

const BACKGROUND_COLOR: Color = [0.01, 0.01, 0.01, 1.0];

fn main() {
	let (width, height) = (61, 31);
	let mut window: PistonWindow = WindowSettings::new("Pong", [to_coord_u32(width), to_coord_u32(height)])
		.exit_on_esc(true)
		.build()
		.unwrap();

	let mut game = Game::new(width, height);
	let mut pressed_keys = HashSet::new();

	while let Some(e) = window.next() {
		if let Some(Button::Keyboard(key)) = e.press_args() {
			pressed_keys.insert(key);
			game.key_pressed(key);
		}

		if let Some(Button::Keyboard(key)) = e.release_args() {
			pressed_keys.remove(&key);
			game.key_pressed(Key::Unknown);
		}

		window.draw_2d(&e, |c, g, _| {
			clear(BACKGROUND_COLOR, g);
			game.draw(&c, g);
		});

		e.update(|args| {
			game.update(args.dt);

			for key in &pressed_keys {
				game.key_pressed(*key);
			}
		});
	}
}
