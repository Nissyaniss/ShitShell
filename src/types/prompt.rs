use std::{env, fmt::Display, io};

use crossterm::{
	cursor::{RestorePosition, SavePosition},
	ExecutableCommand,
};

use crate::{types::cursor::Cursor, types::displaymode::Mode, types::utils::print_flush};

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
				let command_output = env::var("PWD");
				command_output.map_or_else(
					|_| "PWD ERROR > ".to_string(),
					|mut current_path| {
						if let Ok(home) = env::var("HOME") {
							if current_path.starts_with(&home) {
								current_path = current_path.replace(&home, "~");
							}
						}
						format!("{current_path} > ")
					},
				)
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
			cursor.update(false);
		}
	}

	fn update_current_path(&mut self) {
		let home_dir = env::var("PWD");
		self.current_path = home_dir.map_or_else(
			|_| "PWD ERROR > ".to_string(),
			|mut current_path| {
				if let Ok(home) = env::var("HOME") {
					if current_path.starts_with(&home) {
						current_path = current_path.replace(&home, "~");
					}
				}
				current_path
			},
		);
		self.prompt = format!("{} > ", self.current_path);
	}

	pub fn len(&self) -> usize {
		self.prompt.len()
	}
}
