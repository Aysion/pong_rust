use piston_window::types::Color;
use piston_window::{ Context, G2d };

use crate::draw::draw_block;

const COLOR: Color = [0.0, 1.0, 0.0, 1.0];

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
	Up,
	Down,
	None,
}

pub struct Player {
	direction: Direction,
	body: Vec<(i32, i32)>,
}

impl Player {
	pub fn new(x: i32, y: i32) -> Player {
		let mut body: Vec<(i32, i32)> = Vec::new();
		for i in 0..4 {
			body.push((x + i, y));
		}

		Player {
			direction: Direction::None,
			body,
		}
	}

	pub fn draw(&self, con: &Context, g: &mut G2d) {
		for block in &self.body {
			draw_block(COLOR, block.0, block.1, con, g);
		}
	}

	pub fn update(&mut self) {
		let mut new_head = *self.body.first().unwrap();

		match self.direction {
			Direction::Up => new_head.1 -= 1,
			Direction::Down => new_head.1 += 1,
			Direction::None => return,
		}

		self.body.pop();
		self.body.insert(0, new_head);
	}
}
