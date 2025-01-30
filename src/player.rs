use piston_window::types::Color;
use piston_window::{ Context, G2d };

use crate::draw::draw_block;

const COLOR: Color = [1.0, 1.0, 1.0, 1.0];

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Direction {
	Up,
	Down,
	None,
}

pub struct Player {
	pub direction: Direction,
	pub x: i32,
	pub y: i32,
	pub height: i32,
	wait_time: f32,
	speed: f32,
}

impl Player {
	pub fn new(x: i32, y: i32, speed: f32) -> Player {
		Player {
			direction: Direction::None,
			x,
			y,
			height: 5,
			speed,
			wait_time: 0.0,
		}
	}

	pub fn draw(&self, con: &Context, g: &mut G2d) {
		for i in 0..self.height {
			draw_block(COLOR, self.x, self.y + i, con, g);
		}
	}

	pub fn update(&mut self, height_window: i32, delta_time: f64) {
		self.wait_time += delta_time as f32;

		if self.wait_time > self.speed {
			self.wait_time = 0.0;
			self.moving_press(self.direction, height_window);
		}
	}

	pub fn update_ai(&mut self,  height_window: i32, ball_y: i32, delta_time: f64) {
		self.wait_time += delta_time as f32;

		let target_y = ball_y - (self.height / 2);

		if self.y < target_y {
			self.change_direction(Direction::Down);
		} else if self.y > target_y {
			self.change_direction(Direction::Up);
		}

		self.update(height_window, delta_time);
	}

	pub fn change_direction(&mut self, d: Direction) {
		self.direction = d;
	}

	pub fn moving_press(&mut self, d: Direction, height_window: i32) {
		if d == Direction::None {
			return;
		}

		if d == Direction::Up {
			if self.y == 1 {
				return;
			} else {
				self.y -= 1;
			}
		}

		if d == Direction::Down {
			if self.y > height_window - self.height - 2 {
				return;
			} else {
				self.y += 1;
			}
		}
	}
}
