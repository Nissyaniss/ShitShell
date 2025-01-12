use std::fmt::Display;

use crate::{displaymode::Mode, shell::Shell, utils::print_flush};

pub struct Prompt {
	prompt: String,
}

impl Default for Prompt {
	fn default() -> Self {
		Self {
			prompt: String::from("> "),
		}
	}
}

impl Display for Prompt {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.prompt)
	}
}

impl Prompt {
	pub fn display(&self, mode: Mode, command: Option<String>) {
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

	pub fn len(&self) -> usize {
		self.prompt.len()
	}
}
