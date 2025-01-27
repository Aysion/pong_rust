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
	direction: Direction,
	body: Vec<(i32, i32)>,
	height_window: i32,
	wait_time: f32,
	speed: f32,
}

impl Player {
	pub fn new(x: i32, y: i32, height_window: i32, speed: f32) -> Player {
		let mut body: Vec<(i32, i32)> = Vec::new();
		for i in 0..5 {
			body.push((x, y + i));
		}

		Player {
			direction: Direction::None,
			body,
			height_window,
			speed,
			wait_time: 0.0,
		}
	}

	pub fn draw(&self, con: &Context, g: &mut G2d) {
		for block in &self.body {
			draw_block(COLOR, block.0, block.1, con, g);
		}
	}

	pub fn update(&mut self, delta_time: f64) {
		self.wait_time += delta_time as f32;

		if self.wait_time > self.speed {
			self.wait_time = 0.0;
			self.moving_press(self.direction);
		}
	}

	pub fn change_direction(&mut self, d: Direction) {
		self.direction = d;
	}

	pub fn moving_press(&mut self, d: Direction) {
		if d == Direction::None {
			return;
		}

		let mut new_head = *self.body.first().unwrap();

		match d {
			Direction::Up => new_head.1 -= 1,
			Direction::Down => new_head.1 += 1,
			Direction::None => (),
		}

		if new_head.1 < 1 || (new_head.1 + self.body.len() as i32) > self.height_window - 1 {
			return;
		}

		for i in 0..self.body.len() {
			self.body[i].1 = new_head.1 + i as i32;
		}

		println!("Player: {:?} - {:?}", d, self.body);
	}
}
