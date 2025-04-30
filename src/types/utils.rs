use crossterm::event::KeyCode::{self, Char};
use crossterm::event::{KeyEvent, KeyModifiers};

#[macro_export]
macro_rules! print_flush {
    ($($arg:tt)*) => {
    	use std::io::Write;
        print!($($arg)*);
        std::io::stdout().flush().expect("Failed to flush stdout");
    };
}

pub trait KeyEventUtilities {
	fn get_char(&self) -> Option<char>;
	fn has_modifier(&self, modifier: KeyModifiers) -> Option<KeyEvent>;
	fn is_key(&self, key: KeyCode) -> bool;
}

impl KeyEventUtilities for KeyEvent {
	fn get_char(&self) -> Option<char> {
		"qwertyuiopasdfghjklzxcvbnm,./;'[]1234567890QWERTYUIOPASDFGHJKLZXCVBNM+-=!@#$%^&*()_+{}:?\"<>?`~"
			.to_string()
			.chars()
			.find(|&char| self.code == Char(char))
	}

	fn has_modifier(&self, modifier: KeyModifiers) -> Option<KeyEvent> {
		if self.modifiers == modifier {
			Some(*self)
		} else {
			None
		}
	}

	fn is_key(&self, key: KeyCode) -> bool {
		self.code == key
	}
}

pub trait OptionKeyEventUtilities {
	fn is_key(&self, key: KeyCode) -> bool;
	fn is_a_character(&self) -> bool;
}

impl OptionKeyEventUtilities for Option<KeyEvent> {
	fn is_key(&self, key: KeyCode) -> bool {
		self.is_some() && self.unwrap().code == key
	}

	fn is_a_character(&self) -> bool {
		for char in "qwertyuiopasdfghjklzxcvbnm,./;'[]1234567890QWERTYUIOPASDFGHJKLZXCVBNM-=!@#$%^&*()_+{}:?\"<>?`~"
			.to_string()
			.chars()
		{
			if self.is_some() && self.unwrap().code == Char(char) {
				return true;
			}
		}
		false
	}
}
