use crate::types::command::Command;

pub fn parse(command: &str) -> Vec<Command> {
	let mut res = Vec::new();

	let ands = command.split("&&");

	for commands in ands {
		let mut command = Command::default();
		let mut spaces = commands.split(' ');
		let buf = spaces.next().unwrap().to_owned();
		if buf.is_empty() {
			command.set_command(spaces.next().unwrap().to_owned());
		} else {
			command.set_command(buf);
		}
		for word in spaces {
			if !word.is_empty() {
				command.add_arg(word.to_string());
			}
		}
		res.push(command);
	}

	res
}
