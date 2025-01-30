use piston_window::types::Color;
use piston_window::{ Context, G2d };
use rand::{thread_rng, Rng};

use crate::draw::draw_block;
use crate::player::{Direction, Player};

const COLOR: Color = [0.0, 0.0, 0.3, 1.0];

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum BallDirection {
	Left,
	Right,
}

pub struct Ball {
	pub started: bool,
	pub x: i32,
	pub y: i32,
	pub size: i32,
	pub x_speed: i32,
	pub y_speed: i32,
	pub speed: f32,
	pub wait_time: f32,
	pub direction: BallDirection,
}

impl Ball {
	pub fn new(x: i32, y: i32, size: i32, x_speed: i32, y_speed: i32) -> Ball {
		Ball {
			started: false,
			wait_time: 0.0,
			x,
			y,
			size,
			x_speed,
			y_speed,
			speed: 0.05,
			direction: BallDirection::Left,
		}
	}

	pub fn draw(&self, con: &Context, g: &mut G2d) {
		draw_block(COLOR, self.x, self.y, con, g);
	}

	pub fn hit_wall(&mut self, width: i32, height: i32) -> bool {
		if self.x <= 1 || self.x >= (width - 2) {
			return true;
		}

		if self.y <= 1 {
				self.y_speed = self.y_speed.abs();
		} else if self.y >= (height - 2) {
			self.y_speed = -self.y_speed.abs();
		}

		return false;
	}

	pub fn hit_player(&mut self, player: &Player) {
		let hited = if player.x == 1 {
			self.x == player.x + 1 && self.y >= player.y - 1 && self.y <= player.y + player.height
		} else {
			self.x == player.x - 1 && self.y >= player.y - 1 && self.y <= player.y + player.height
		};

		if hited {
			match player.direction {
				Direction::Up => self.y_speed += if self.y_speed > 0 { if self.y_speed == 1 { 0 } else { -1 } } else { -1 },
				Direction::Down => self.y_speed += if self.y_speed > 0 { 1 } else { if self.y_speed == -1 { 0 } else { 1 } },
				_ => (),
			}

			self.change_direction();
		}
	}

	pub fn change_direction(&mut self) {
		self.direction = match self.direction {
			BallDirection::Left => BallDirection::Right,
			BallDirection::Right => BallDirection::Left,
		};
	}

	pub fn start_direction(&mut self) {
		let mut rng = thread_rng();
		let direction = rng.gen_range(0..2);
		self.direction = if direction == 0 {
			BallDirection::Left
		} else {
			BallDirection::Right
		};

		self.started = true;
	}

	pub fn reset(&mut self, x: i32, y: i32, x_speed: i32, y_speed: i32) {
		self.x = x;
		self.y = y;
		self.x_speed = x_speed;
		self.y_speed = y_speed;
		self.started = false;
	}

	pub fn update(&mut self, delta_time: f64) {
		if !self.started {
			return;
		}

		self.wait_time += delta_time as f32;

		if self.wait_time < self.speed {
			return;
		}

		self.wait_time = 0.0;

		match self.direction {
			BallDirection::Left => self.x -= self.x_speed,
			BallDirection::Right => self.x += self.x_speed,
		}

		self.y += self.y_speed;
	}
}
