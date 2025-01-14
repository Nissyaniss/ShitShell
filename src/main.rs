mod builtin_commands;
mod command;
mod displaymode;
mod exitcode;
mod history;
mod prompt;
mod utils;

use std::{io, ops::Index};

use command::Command;
use crossterm::{
	event::{
		Event,
		KeyCode::{self, Backspace, Char, Down, Enter, Up},
		KeyEvent, KeyEventKind, KeyModifiers,
	},
	terminal::{disable_raw_mode, enable_raw_mode},
};
use displaymode::Mode;
use history::History;
use prompt::Prompt;
use utils::{print_flush, KeyEventUtilities, OptionKeyEventUtilities};

#[allow(non_upper_case_globals)]
const Space: KeyCode = Char(' ');

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

fn handle_history(
	event: KeyEvent,
	history: &mut History,
	current_command: &mut Command,
	prompt: &mut Prompt,
) {
	if event.is_key(Up) && history.items.len() > history.current_index && !history.items.is_empty()
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
					.len() + prompt.len()
					+ 1,
			);
		}
		prompt.display(Mode::DisplayCommand, Some(current_command.to_string()));
	} else if event.is_key(Down)
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
					.len() + prompt.len()
					+ 1,
			);
		}
		prompt.display(Mode::DisplayCommand, Some(current_command.to_string()));
	} else if event.is_key(Down) && history.current_index.saturating_sub(1) == 0 {
		*current_command = Command::default();
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
					.len() + prompt.len()
					+ 3,
			);
		}
		prompt.display(Mode::DisplayCommand, Some(current_command.to_string()));
		history.current_index = 0;
	}
}

fn print_events() -> io::Result<()> {
	let mut current_command = Command::default();
	let mut history = History {
		items: Vec::new(),
		current_index: 0,
	};
	let mut prompt = Prompt::default();
	prompt.display(Mode::Normal, None);
	loop {
		let event = crossterm::event::read()?;
		match event {
			Event::Key(event) if event.kind == KeyEventKind::Press => {
				if event.has_modifier(KeyModifiers::CONTROL).is_key(Char('d')) {
					break;
				} else if event.is_key(Enter) {
					current_command.handle_command()?;
					enable_raw_mode()?;
					if !current_command.is_empty() {
						history.push(current_command);
					}
					current_command = Command::default();
					prompt.display(Mode::CarriageReturn, None);
					history.current_index = 0;
				} else if event.has_modifier(KeyModifiers::CONTROL).is_key(Char('c')) {
					current_command = Command::default();
					prompt.display(Mode::NewLineAndCarriageReturn, None);
				} else if event.is_key(Backspace) {
					if !current_command.is_empty() {
						prompt.display(Mode::Backspace, Some(current_command.to_string()));
						current_command.pop();
					}
				} else if event.is_key(Space) {
					current_command.push(' ');
					prompt.display(Mode::DisplayCommand, Some(current_command.to_string()));
				} else if event.has_modifier(KeyModifiers::empty()).is_a_character()
					|| event.has_modifier(KeyModifiers::SHIFT).is_a_character()
				{
					current_command.push(event.get_char().unwrap());
					print_flush(&format!("{}", event.code));
				} else if event.is_key(Up) || event.is_key(Down) {
					handle_history(event, &mut history, &mut current_command, &mut prompt);
				}
			}
			_ => {}
		}
	}
	Ok(())
}
