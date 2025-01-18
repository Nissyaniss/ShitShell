use crossterm::terminal::disable_raw_mode;
use std::{fmt::Display, io::Result, process::Command as ProcessCommand};

use crate::{builtin_commands::cd::cd, utils::print_flush};

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
		disable_raw_mode().unwrap();
		let mut input = self.command_string.split_whitespace();
		let command_string = match input.next() {
			Some(string) => string.trim(),
			None => return Ok(0),
		};
		let args_len: usize = input.count();
		let mut args = self.command_string.split_whitespace();
		args.next();
		if command_string == "cd" {
			if args_len == 0 {
				cd("");
			} else {
				cd(args.next().unwrap());
			}
			Ok(0)
		} else {
			let command = ProcessCommand::new(command_string)
				.args(args.clone())
				.spawn();
			if let Ok(mut command) = command {
				print_flush("\r\n");
				Ok(command.wait()?.code().unwrap_or(0))
			} else {
				print_flush(&format!("\r\n{command_string}: Not a command\n"));
				Ok(0)
			}
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
