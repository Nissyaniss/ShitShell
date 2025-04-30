use crossterm::terminal::disable_raw_mode;
use std::{
	fmt::Display,
	io::Result,
	process::Command as ProcessCommand,
	sync::{
		atomic::{AtomicBool, Ordering},
		Arc,
	},
};

use crate::{builtin_commands::cd::cd, print_flush};

#[derive(Default)]
pub struct Command {
	command_string: String,
}

impl Command {
	pub fn len(&self) -> usize {
		self.command_string.len()
	}

	pub fn handle_command(&self) -> Result<i32> {
		let running = Arc::new(AtomicBool::new(true));
		let r = running.clone();

		signal_hook::flag::register(signal_hook::consts::SIGINT, running)?;

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
				cd("~");
			} else {
				cd(args.next().unwrap());
			}
			Ok(0)
		} else {
			let command = ProcessCommand::new(command_string)
				.args(args.clone())
				.spawn();
			print_flush!("\n");
			let mut res = 1;
			command.map_or_else(
				|_| {
					print_flush!("\r\n{command_string}: Not a command\n");
					Ok(0)
				},
				|mut command| {
					while command.try_wait().unwrap().is_none() {
						match command.try_wait() {
							Ok(Some(status)) => {
								res = status.code().unwrap();
							}
							Ok(None) => {
								if !r.load(Ordering::Relaxed) {
									let _ = command.kill();
									let _ = command.wait();
									r.store(true, Ordering::Relaxed);
								}
								std::thread::sleep(std::time::Duration::from_millis(100));
								res = 1;
							}
							Err(e) => {
								eprintln!("Error waiting for command: {e}");
								res = 1;
							}
						}
					}
					Ok(res)
				},
			)
		}
	}

	pub fn is_empty(&self) -> bool {
		self.command_string.is_empty()
	}

	pub fn pop(&mut self) -> Option<char> {
		self.command_string.pop()
	}

	pub fn insert(&mut self, char: char, index: usize) {
		self.command_string.insert(index, char);
	}

	pub fn set(&mut self, command: String) {
		self.command_string = command;
	}

	pub fn remove(&mut self, index: usize) {
		self.command_string.remove(index);
	}
}

impl Display for Command {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.command_string)
	}
}
