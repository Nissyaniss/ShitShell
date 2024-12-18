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
	terminal::{disable_raw_mode, enable_raw_mode},
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
		println!("Error: {e:?}\r");
	}
	disable_raw_mode()?;
	Ok(())
}

fn print_flush(string: &str) {
	print!("{string}");
	let _ = stdout().flush();
}

fn clear_prompt() {
	print!("{}", " ".repeat(15)); //BAD BUT DONT KNOW FOR NOW SO TODO
	let _ = stdout().flush();
}

#[allow(clippy::option_if_let_else)]
fn handle_command(input: &str) -> i32 {
	let mut input = input.split_whitespace();
	let command_string = match input.next() {
		Some(string) => string.trim(),
		None => return 0,
	};
	let args = input;
	let command = Command::new(command_string).args(args.clone()).spawn();
	if let Ok(mut command) = command {
		print_flush("\r\n");
		command.wait().unwrap().code().unwrap()
	} else {
		print_flush(&format!("\r\n{command_string}: Not a command"));
		0
	}
}

fn print_events() -> io::Result<()> {
	let prompt = String::from("> ");
	let mut command = String::new();
	let history_data = fs::read_to_string("shitshell_history").unwrap_or_else(|_| {
		let _ = File::create("./shitshell_history"); //Bad
		String::new()
	});
	let mut history: Vec<String> = history_data
		.split('\n')
		.map(std::string::ToString::to_string)
		.collect();
	let mut history_index: usize = history.len() - 1;
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
					let result = handle_command(&command);
					history.push(command);
					history_index = history.len() - 1;
					command = String::new();
					if result == 0 {
						print_flush(&format!("\n\r{prompt}"));
					} else {
						print_flush(&format!("\r{prompt}"));
					}
				//CTRL+c to cancel (not done)
				} else if event.code == Char('c') && event.modifiers == KeyModifiers::CONTROL {
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
				//History up
				} else if event.code == Up {
					history_index = history_index.saturating_sub(1);
					if history.len() > history_index {
						command = history.index(history_index).to_string();
						clear_prompt();
						print_flush(&format!("\r{prompt}{command}"));
					}
				//History Down
				} else if event.code == Down {
					history_index += 1;
					if history_index < history.len() {
						command = history.index(history_index).to_string();
						clear_prompt();
						print_flush(&format!("\r{prompt}{command}"));
					}
				}
			}
			_ => {}
		}
	}
	Ok(())
}
