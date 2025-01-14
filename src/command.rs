use crossterm::terminal::disable_raw_mode;
use std::{
	env::{set_current_dir, set_var},
	fmt::Display,
	io::Result,
	path::Path,
	process::Command as ProcessCommand,
};

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
		let mut args_len: usize = 0;
		let _ = args.clone().inspect(|_| {
			args_len += 1;
		});
		disable_raw_mode().unwrap();
		if command_string == "cd" {
			if args_len == 0 {
				let yay = set_current_dir(Path::new("/home/nissya"));
				if yay.is_err() {
					print_flush(&format!("{}", yay.err().unwrap()));
				}
				print_flush("\r\n");
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
