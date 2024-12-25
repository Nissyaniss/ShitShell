use std::{
	fs::{self, File},
	io::{self, stdout, Write},
	ops::Index,
	process::Command,
};

use crossterm::{
	event::{
		Event,
		KeyCode::{Backspace, Char, Down, Enter, Up},
		KeyEvent, KeyEventKind, KeyModifiers,
	},
	terminal::{disable_raw_mode, enable_raw_mode, size},
};

trait CharacterUtils {
	fn is_a_character(&self) -> bool;
	fn get_char(&self) -> Option<char>;
}

impl CharacterUtils for KeyEvent {
	fn is_a_character(&self) -> bool {
		for char in "qwertyuiopasdfghjklzxcvbnm,./;'[]1234567890-=!@#$%^&*()_+{}:?\"<>?`~"
			.to_string()
			.chars()
		{
			if self.code == Char(char) {
				return true;
			}
		}
		false
	}

	fn get_char(&self) -> Option<char> {
		"qwertyuiopasdfghjklzxcvbnm,./;'[]1234567890-=!@#$%^&*()_+{}:?\"<>?`~"
			.to_string()
			.chars()
			.find(|&char| self.code == Char(char))
	}
}

fn main() -> io::Result<()> {
	enable_raw_mode()?;
	if let Err(e) = print_events() {
		print!("\nError: {e:?}\r");
	}
	disable_raw_mode()?;
	Ok(())
}

fn print_flush(string: &str) {
	print!("{string}");
	let _ = stdout().flush();
}

fn clear_line(len: usize) {
	print_flush(&format!("\r{}", &" ".repeat(len)));
}

#[allow(clippy::option_if_let_else)]
fn handle_command(input: &str) -> io::Result<i32> {
	let mut input = input.split_whitespace();
	let command_string = match input.next() {
		Some(string) => string.trim(),
		None => return Ok(0),
	};
	let args = input;
	disable_raw_mode().unwrap();
	let command = Command::new(command_string).args(args.clone()).spawn();
	if let Ok(mut command) = command {
		print_flush("\r\n");
		Ok(command.wait()?.code().unwrap_or(0))
	} else {
		print_flush(&format!("\r\n{command_string}: Not a command"));
		Ok(0)
	}
}

fn print_events() -> io::Result<()> {
	let mut command = String::new();
	let mut history = Vec::new();
	let mut history_index = 0;
	let prompt = String::from("> ");
	print_flush(&prompt);
	loop {
		let event = crossterm::event::read()?;
		match event {
			Event::Key(event) if event.kind == KeyEventKind::Press => {
				//Bad code but will do for now
				//CTRL+d to quit
				if event.code == Char('d') && event.modifiers == KeyModifiers::CONTROL {
					break;
				//Enter to validate
				} else if event.code == Enter {
					command = String::new();
					if result == 0 {
						print_flush(&format!("\n\r{prompt}"));
					} else {
						print_flush(&format!("\r{prompt}"));
					}
					history_index = 0;
				} else if event.code == Char('c') && event.modifiers == KeyModifiers::CONTROL {
					command = String::new();
					print_flush(&format!("\n\r{prompt}"));
				//Backspace
				} else if event.code == Backspace {
					if command.is_empty() {
						print_flush(&format!("\r{prompt}"));
					} else {
						print_flush(&format!("\r{prompt}{command}\x08 \x08"));
					}
					command.pop();
				//Space
				} else if event.code == Char(' ') {
					command.push(' ');
					print_flush(&format!("\r{prompt}{command}"));
				//Characters
				} else if event.is_a_character() && event.modifiers == KeyModifiers::empty() {
					command.push(event.get_char().unwrap());
					print_flush(&format!("{}", event.code));
				} else if event.code == Up && history.len() > history_index && !history.is_empty() {
					history_index += 1;
					command = history
						.index(history.len().saturating_sub(history_index))
						.to_string();
					if history_index > 1 {
						clear_line(
							history
								.index(history.len().saturating_sub(history_index - 1))
								.len() + prompt.len() + 1,
						);
					}
					print_flush(&format!("\r{prompt}{command}"));
				} else if event.code == Down
					&& history_index.saturating_sub(1) > 0
					&& history.len() > history_index - 1
					&& !history.is_empty()
				{
					history_index -= 1;
					command = history
						.index(history.len().saturating_sub(history_index))
						.to_string();
					if history.len() > history_index {
						clear_line(
							history
								.index(history.len().saturating_sub(history_index + 1))
								.len() + prompt.len() + 1,
						);
					}
					print_flush(&format!("\r{prompt}{command}"));
				} else if event.code == Down && history_index.saturating_sub(1) == 0 {
					command = String::new();
					if history.len() > history_index {
						clear_line(
							history
								.index(history.len().saturating_sub(history_index + 1))
								.len() + prompt.len() + 2,
						);
					}
					print_flush(&format!("\r{prompt}{command}"));
				}
			}
			_ => {}
		}
	}
	Ok(())
}
