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

use crate::{builtin_commands::cd::cd, parser, print_flush};

#[derive(Default, Debug)]
pub struct Command {
	command: String,
	args: Vec<String>,
}

fn exec_command(command: &Command, is_first_time: bool) -> Result<i32> {
	let running = Arc::new(AtomicBool::new(true));
	let r = running.clone();

	signal_hook::flag::register(signal_hook::consts::SIGINT, running)?;

	disable_raw_mode().unwrap();

	let mut input = command.command.split_whitespace();
	let command_string = match input.next() {
		Some(string) => string.trim(),
		None => return Ok(0),
	};
	let args = command.get_args();
	if command_string == "cd" {
		cd(args);
		Ok(0)
	} else {
		let command = ProcessCommand::new(command_string).args(args).spawn();
		let mut res = 1;
		if is_first_time {
			print_flush!("\n");
		}
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

impl Command {
	pub fn len(&self) -> usize {
		self.command.len()
	}

	pub fn handle_commands(command_str: &str) -> Result<i32> {
		let commands = parser::parse(command_str);
		let mut res = 1;
		let mut is_first_time = true;
		for command in commands {
			if is_first_time {
				res = exec_command(&command, true)?;
				is_first_time = false;
				continue;
			}
			res = exec_command(&command, false)?;
		}
		Ok(res)
	}

	pub fn is_empty(&self) -> bool {
		self.command.is_empty()
	}

	pub fn pop(&mut self) -> Option<char> {
		self.command.pop()
	}

	pub fn insert(&mut self, char: char, index: usize) {
		self.command.insert(index, char);
	}

	pub fn set_command(&mut self, command: String) {
		self.command = command;
	}

	pub fn remove(&mut self, index: usize) {
		self.command.remove(index);
	}

	pub fn add_arg(&mut self, arg: String) {
		self.args.push(arg);
	}

	pub fn get_command(&self) -> &str {
		&self.command
	}

	pub fn get_args(&self) -> &[String] {
		&self.args
	}
}

impl Display for Command {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.command)
	}
}
