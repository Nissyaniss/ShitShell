use std::{fmt::Display, process::Command};

use crate::{displaymode::Mode, utils::print_flush};

pub struct Prompt {
	prompt: String,
	current_path: String,
}

impl Default for Prompt {
	fn default() -> Self {
		Self {
			current_path: String::new(),
			prompt: { "~ > ".to_string() },
		}
	}
}

impl Display for Prompt {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.prompt)
	}
}

impl Prompt {
	pub fn display(&mut self, mode: Mode, command: Option<String>) {
		self.update_current_path();
		match mode {
			Mode::CarriageReturn => print_flush(&format!("\r{}", self.prompt)),
			Mode::NewLineAndCarriageReturn => print_flush(&format!("\n\r{}", self.prompt)),
			Mode::DisplayCommand => {
				if command.is_some() {
					print_flush(&format!("\r{}{}", self.prompt, command.unwrap()));
				}
			}
			Mode::Backspace => {
				if command.is_some() {
					print_flush(&format!("\r{}{}\x08 \x08", self.prompt, command.unwrap()));
				}
			}
			Mode::Normal => print_flush(&self.prompt),
		}
	}

	pub fn update_current_path(&mut self) {
		let command_output = Command::new("pwd").output();
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
