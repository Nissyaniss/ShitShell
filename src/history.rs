use crate::command::Command;

pub struct History {
	pub items: Vec<Command>,
	pub current_index: usize,
}

impl History {
	pub fn push(&mut self, command: Command) {
		self.items.push(command);
	}
}
