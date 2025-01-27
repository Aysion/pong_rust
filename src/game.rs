use piston_window::*;
use piston_window::types::Color;

use rand::{thread_rng, Rng};

use crate::draw::{ draw_rectangle, draw_dashed_line };
use crate::player::{ Player, Direction };

const BORDER_COLOR: Color = [0.3, 0.3, 0.3, 1.0];
const GAMEOVER_COLOR: Color = [0.8, 0.0, 0.0, 0.5];

const MOVING_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 3.0;

pub struct Game {
	width: i32,
	height: i32,
	game_over: bool,
	wait_time: f64,
	player1: Player,
	player2: Player,
}

impl Game {
	pub fn new(width: i32, height: i32) -> Game {
		Game {
			width,
			height,
			game_over: false,
			wait_time: 0.0,
			player1: Player::new(1, height / 2 - 2, height, 0.01 as f32),
			player2: Player::new(width - 2, height / 2 - 2, height, 0.6 as f32),
		}
	}

	pub fn draw(&self, con: &Context, g: &mut G2d) {
		draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
		draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
		draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
		draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);
		draw_dashed_line(BORDER_COLOR, self.width / 2, 0, 1, self.height, con, g);

		self.player1.draw(con, g);
		self.player2.draw(con, g);

		if self.game_over {
			draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);
		}
	}

	pub fn update(&mut self, delta_time: f64) {
		self.wait_time += delta_time;

		if self.game_over {
			if self.wait_time > RESTART_TIME {
				self.restart();
			}

			return;
		}

		if self.wait_time > MOVING_PERIOD {
			self.wait_time = 0.0;
		}

		self.player1.update(delta_time);
	}

	pub fn restart(&mut self) {
		self.game_over = false;
		self.wait_time = 0.0;
	}

	pub fn key_pressed(&mut self, key: Key) {
		if self.game_over {
			return;
		}

		let new_dir = match key {
			Key::Up => Some(Direction::Up),
			Key::Down => Some(Direction::Down),
			_ => Some(Direction::None),
		};

		self.player1.change_direction(new_dir.unwrap());
	}
}
