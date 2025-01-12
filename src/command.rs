use std::{fmt::Display, io::Result, process::Command as ProcessCommand};

use crossterm::terminal::disable_raw_mode;

use crate::utils::print_flush;

#[derive(Default)]
pub struct Command {
	command_string: String,
}

impl Command {
	pub fn len(&self) -> usize {
		self.command_string.len()
	}

	#[allow(clippy::option_if_let_else)]
	pub fn handle_command(&self) -> Result<i32> {
		let mut input = self.command_string.split_whitespace();
		let command_string = match input.next() {
			Some(string) => string.trim(),
			None => return Ok(0),
		};
		let args = input;
		disable_raw_mode().unwrap();
		let command = ProcessCommand::new(command_string)
			.args(args.clone())
			.spawn();
		if let Ok(mut command) = command {
			print_flush("\r\n");
			Ok(command.wait()?.code().unwrap_or(0))
		} else {
			print_flush(&format!("\r\n{command_string}: Not a command"));
			Ok(0)
		}
	}

	pub fn is_empty(&self) -> bool {
		self.command_string.is_empty()
	}

	pub fn pop(&mut self) {
		self.command_string.pop();
	}

	pub fn push(&mut self, char: char) {
		self.command_string.push(char);
	}

	pub fn set(&mut self, command: String) {
		self.command_string = command;
	}
}

impl Display for Command {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.command_string)
	}
}
