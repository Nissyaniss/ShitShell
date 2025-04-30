use crate::types::command::Command;

pub fn parse(command: &str) -> Vec<Command> {
	let mut res = Vec::new();

	let ands = command.split("&&");

	for commands in ands {
		let mut command = Command::default();
		let mut spaces = commands.split(' ');
		let buf = spaces.next().unwrap().to_owned();
		if buf.is_empty() {
			match spaces.next() {
				Some(command_string) => command.set_command(command_string.to_string()),
				None => continue,
			}
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
