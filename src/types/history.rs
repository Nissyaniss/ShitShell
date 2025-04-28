use std::ops::Index;

use crossterm::event::{
	KeyCode::{Down, Up},
	KeyEvent,
};

use crate::types::command::Command;

use super::{
	cursor::Cursor,
	displaymode::Mode,
	prompt::{clear_line, Prompt},
	utils::KeyEventUtilities,
};

pub struct History {
	pub items: Vec<Command>,
	pub current_index: usize,
}

impl History {
	pub fn push(&mut self, command: Command) {
		self.items.push(command);
	}

	pub fn handle_history(
		&mut self,
		event: KeyEvent,
		current_command: &mut Command,
		prompt: &mut Prompt,
		cursor: &mut Cursor,
	) {
		if event.is_key(Up) && self.items.len() > self.current_index && !self.items.is_empty() {
			self.current_index += 1;
			current_command.set(
				self.items
					.index(self.items.len().saturating_sub(self.current_index))
					.to_string(),
			);
			if self.current_index > 1 {
				clear_line(
					self.items
						.index(self.items.len().saturating_sub(self.current_index - 1))
						.len() + prompt.len()
						+ 1,
				);
			}
			prompt.display(Mode::Normal, Some(current_command.to_string()), cursor);
		} else if event.is_key(Down)
			&& self.current_index.saturating_sub(1) > 0
			&& self.items.len() > self.current_index - 1
			&& !self.items.is_empty()
		{
			self.current_index -= 1;
			current_command.set(
				self.items
					.index(self.items.len().saturating_sub(self.current_index))
					.to_string(),
			);
			if self.items.len() > self.current_index {
				clear_line(
					self.items
						.index(self.items.len().saturating_sub(self.current_index + 1))
						.len() + prompt.len()
						+ 1,
				);
			}
			prompt.display(Mode::Normal, Some(current_command.to_string()), cursor);
		} else if event.is_key(Down) && self.current_index.saturating_sub(1) == 0 {
			*current_command = Command::default();
			if self.items.len() > self.current_index {
				clear_line(
					self.items
						.index(self.items.len().saturating_sub(self.current_index + 1))
						.len() + prompt.len()
						+ 3,
				);
			}
			prompt.display(Mode::Normal, Some(current_command.to_string()), cursor);
			self.current_index = 0;
		}
	}
}
