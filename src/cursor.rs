use std::io;

use crossterm::{
	cursor::{position, MoveLeft, MoveRight, MoveTo},
	ExecutableCommand,
};

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct Position {
	pub row: u16,
	pub line: u16,
}

pub struct Cursor {
	pub initial_position: Position,
	pub position: Position,
	pub has_moved: bool,
}

impl Cursor {
	pub const fn new(position: Position) -> Self {
		Self {
			initial_position: position,
			position,
			has_moved: false,
		}
	}

	pub fn move_left(&mut self) {
		if self.position.row.saturating_sub(1) >= self.initial_position.row {
			self.position.row = self.position.row.saturating_sub(1);
			let _ = io::stdout().execute(MoveLeft(1));
			self.has_moved = true;
		}
	}

	pub fn move_right(&mut self, bound: u16) {
		if self.position.row <= bound.saturating_sub(1) {
			self.position.row = self.position.row.saturating_add(1);
			let _ = io::stdout().execute(MoveRight(1));
			self.has_moved = true;
		}
	}

	pub fn update(&mut self, do_initial_update: bool) {
		let cursor_position = position().unwrap();
		self.position.row = cursor_position.0;
		self.position.line = cursor_position.1;
		self.has_moved = false;

		if do_initial_update {
			self.initial_position = self.position;
		}
	}

	pub fn move_to(&mut self, position: Position) {
		let _ = io::stdout().execute(MoveTo(position.row, position.line));
		self.update(false);
	}
}
