use std::{fmt::Display, io, process::Command};

use crossterm::{
	cursor::{RestorePosition, SavePosition},
	ExecutableCommand,
};

use crate::{cursor::Cursor, displaymode::Mode, utils::print_flush};

pub struct Prompt {
	prompt: String,
	current_path: String,
}

impl Default for Prompt {
	fn default() -> Self {
		Self {
			current_path: String::new(),
			prompt: {
				// This whole block isn't really good but only solution i can came up with
				let command_output = Command::new("pwd").output(); //TODO: Use the env instead
				if command_output.is_ok() {
					let mut current_path =
						String::from_utf8(command_output.unwrap().stdout).unwrap();
					current_path.pop();
					format!("{current_path} > ")
				} else {
					"~ > ".to_string()
				}
			},
		}
	}
}

impl Display for Prompt {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.prompt)
	}
}

impl Prompt {
	pub fn display(&mut self, mode: Mode, command: Option<String>, cursor: &mut Cursor) {
		if cursor.has_moved {
			let _ = io::stdout().execute(SavePosition);
		}
		self.update_current_path();
		match mode {
			Mode::CarriageReturn => print_flush(&format!("\r{}", self.prompt)),
			Mode::NewLineAndCarriageReturn => print_flush(&format!("\n\r{}", self.prompt)),
			Mode::Normal | Mode::Backspace => {
				if command.is_some() {
					print_flush(&format!("\r{}{}", self.prompt, command.unwrap()));
				}
			}
		}
		if cursor.has_moved {
			let _ = io::stdout().execute(RestorePosition);
			if mode == Mode::Backspace {
				cursor.move_left();
			}
		} else {
			cursor.update();
		}
	}

	pub fn update_current_path(&mut self) {
		let command_output = Command::new("pwd").output(); //TODO: Use the env instead
		if command_output.is_ok() {
			self.current_path = String::from_utf8(command_output.unwrap().stdout).unwrap();
			self.current_path.pop();
			self.prompt = format!("{} > ", self.current_path);
		}
	}

	pub fn len(&self) -> usize {
		self.prompt.len()
	}
}
