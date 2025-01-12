mod command;
mod displaymode;
mod history;
mod prompt;
mod utils;

use std::{io, ops::Index};

use command::Command;
use crossterm::{
	event::{
		Event,
		KeyCode::{Backspace, Char, Down, Enter, Up},
		KeyEvent, KeyEventKind, KeyModifiers,
	},
	terminal::{disable_raw_mode, enable_raw_mode},
};
use displaymode::Mode;
use history::History;
use prompt::Prompt;
use utils::{print_flush, Characters};

impl Characters for KeyEvent {
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

fn clear_line(len: usize) {
	print_flush(&format!("\r{}", &" ".repeat(len)));
}

fn print_events() -> io::Result<()> {
	let mut current_command = Command::default();
	let mut history = History {
		items: Vec::new(),
		current_index: 0,
	};
	let prompt = Prompt::default();
	prompt.display(Mode::Normal, None);
	loop {
		let event = crossterm::event::read()?;
		match event {
			Event::Key(event) if event.kind == KeyEventKind::Press => {
				if event.code == Char('d') && event.modifiers == KeyModifiers::CONTROL {
					break;
				} else if event.code == Enter {
					current_command.handle_command()?;
					enable_raw_mode()?;
					if !current_command.is_empty() {
						history.push(current_command);
					}
					current_command = Command::default();
					prompt.display(Mode::CarriageReturn, None);
					history.current_index = 0;
				} else if event.code == Char('c') && event.modifiers == KeyModifiers::CONTROL {
					current_command = Command::default();
					prompt.display(Mode::NewLineAndCarriageReturn, None);
				} else if event.code == Backspace {
					if current_command.is_empty() {
						prompt.display(Mode::CarriageReturn, None);
					} else {
						prompt.display(Mode::Backspace, Some(current_command.to_string()));
					}
					current_command.pop();
				} else if event.code == Char(' ') {
					current_command.push(' ');
					prompt.display(Mode::DisplayCommand, Some(current_command.to_string()));
				} else if event.is_a_character() && event.modifiers == KeyModifiers::empty() {
					current_command.push(event.get_char().unwrap());
					print_flush(&format!("{}", event.code));
				} else if event.code == Up
					&& history.items.len() > history.current_index
					&& !history.items.is_empty()
				{
					history.current_index += 1;
					current_command.set(
						history
							.items
							.index(history.items.len().saturating_sub(history.current_index))
							.to_string(),
					);
					if history.current_index > 1 {
						clear_line(
							history
								.items
								.index(
									history
										.items
										.len()
										.saturating_sub(history.current_index - 1),
								)
								.len() + prompt.len() + 1,
						);
					}
					prompt.display(Mode::DisplayCommand, Some(current_command.to_string()));
				} else if event.code == Down
					&& history.current_index.saturating_sub(1) > 0
					&& history.items.len() > history.current_index - 1
					&& !history.items.is_empty()
				{
					history.current_index -= 1;
					current_command.set(
						history
							.items
							.index(history.items.len().saturating_sub(history.current_index))
							.to_string(),
					);
					if history.items.len() > history.current_index {
						clear_line(
							history
								.items
								.index(
									history
										.items
										.len()
										.saturating_sub(history.current_index + 1),
								)
								.len() + prompt.len() + 1,
						);
					}
					prompt.display(Mode::DisplayCommand, Some(current_command.to_string()));
				} else if event.code == Down && history.current_index.saturating_sub(1) == 0 {
					current_command = Command::default();
					if history.items.len() > history.current_index {
						clear_line(
							history
								.items
								.index(
									history
										.items
										.len()
										.saturating_sub(history.current_index + 1),
								)
								.len() + prompt.len() + 2,
						);
					}
					prompt.display(Mode::DisplayCommand, Some(current_command.to_string()));
				}
			}
			_ => {}
		}
	}
	Ok(())
}
