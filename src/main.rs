extern crate rand;
extern crate piston_window;

mod draw;
mod game;
mod player;

use piston_window::*;
use piston_window::types::Color;

use crate::game::Game;
use crate::draw::to_coord_u32;

const BACKGROUND_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
	let (width, height) = (61, 31);
	let mut window: PistonWindow = WindowSettings::new("Pong", [to_coord_u32(width), to_coord_u32(height)])
		.exit_on_esc(true)
		.build()
		.unwrap();

	let mut game = Game::new(width, height);

	while let Some(e) = window.next() {
		if let Some(Button::Keyboard(key)) = e.press_args() {
			game.key_pressed(key);
		}

		window.draw_2d(&e, |c, g, _| {
			clear(BACKGROUND_COLOR, g);
			game.draw(&c, g);
		});

		e.update(|args| {
			game.update(args.dt);
		});
	}
}
