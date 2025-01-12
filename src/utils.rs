use std::io::{stdout, Write};

pub fn print_flush(string: &str) {
	print!("{string}");
	let _ = stdout().flush();
}

pub trait Characters {
	fn get_char(&self) -> Option<char>;
	fn is_a_character(&self) -> bool;
}
