mod builtin_commands;
mod parser;
mod types;

use std::io::{self};

use crossterm::{
	cursor::position,
	event::{
		Event,
		KeyCode::{self, Backspace, Char, Down, Enter, Left, Right, Up},
		KeyEventKind, KeyModifiers,
	},
	terminal::{disable_raw_mode, enable_raw_mode},
};
use types::{
	command::Command,
	cursor::{Cursor, Position},
	displaymode::Mode,
	history::History,
	prompt::{clear_line, Prompt},
	utils::{KeyEventUtilities, OptionKeyEventUtilities},
};

#[allow(non_upper_case_globals)]
const Space: KeyCode = Char(' ');

fn main() -> io::Result<()> {
	enable_raw_mode()?;
	if let Err(e) = shell() {
		print!("\nError: {e:?}\r");
	}
	disable_raw_mode()?;
	Ok(())
	// let res = parser::parse("ls -a -c &&& ls");
	// println!("{res:#?}");
	// Ok(())
}

fn shell() -> io::Result<()> {
	let mut current_command = Command::default();
	let mut history = History {
		items: Vec::new(),
		current_index: 0,
	};
	let mut prompt = Prompt::default();
	print_flush!("{}", &prompt.to_string()); //Not ideal
	let mut cursor = Cursor::new(Position {
		row: position().unwrap().0,
		line: position().unwrap().1,
	});

	loop {
		let event = crossterm::event::read()?;
		match event {
			Event::Key(event) if event.kind == KeyEventKind::Press => {
				if event.has_modifier(KeyModifiers::CONTROL).is_key(Char('d')) {
					break;
				} else if event.is_key(Enter) {
					cursor.has_moved = false;
					Command::handle_commands(current_command.get_command())?;
					enable_raw_mode()?;
					if !current_command.is_empty() {
						history.push(current_command);
					}
					current_command = Command::default();
					prompt.display(Mode::CarriageReturn, None, &mut cursor);
					cursor.update(true);
					history.current_index = 0;
				} else if event.has_modifier(KeyModifiers::CONTROL).is_key(Char('c')) {
					current_command = Command::default();
					prompt.display(Mode::NewLineAndCarriageReturn, None, &mut cursor);
					cursor.has_moved = false;
					cursor.move_to(Position {
						row: TryFrom::<usize>::try_from(prompt.len() + current_command.len())
							.unwrap_or(0),
						line: position().unwrap_or_default().1 + 1,
					});
				} else if event.is_key(Backspace) {
					if !current_command.is_empty() {
						clear_line(prompt.len() + current_command.len()); // Causes flickering but fixes backspace issues need to change in the future
						if current_command.len()
							<= (cursor.position.row - cursor.initial_position.row).into()
						{
							current_command.pop();
						} else if cursor.position.row - cursor.initial_position.row != 0 {
							current_command.remove(
								(cursor.position.row - cursor.initial_position.row - 1).into(),
							);
						}
						prompt.display(
							Mode::Backspace,
							Some(current_command.to_string()),
							&mut cursor,
						);
					}
				} else if event.has_modifier(KeyModifiers::empty()).is_a_character()
					|| event.has_modifier(KeyModifiers::SHIFT).is_a_character()
					|| event.is_key(Space)
				{
					current_command.insert(
						if event.is_key(Space) {
							' '
						} else {
							event.get_char().unwrap()
						},
						(cursor.position.row - cursor.initial_position.row).into(),
					);
					if prompt.len() + current_command.len() != cursor.position.row.into() {
						cursor.move_right(
							TryFrom::<usize>::try_from(prompt.len() + current_command.len())
								.unwrap_or(0),
						);
					}
					prompt.display(Mode::Normal, Some(current_command.to_string()), &mut cursor);
				} else if event.is_key(Up) || event.is_key(Down) {
					history.handle_history(event, &mut current_command, &mut prompt, &mut cursor);
				} else if event.is_key(Left) {
					cursor.move_left();
				} else if event.is_key(Right) {
					cursor.move_right(
						TryFrom::<usize>::try_from(prompt.len() + current_command.len())
							.unwrap_or(0),
					);
				}
			}
			_ => {}
		}
	}
	Ok(())
}
