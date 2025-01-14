use std::{
	env::{set_current_dir, var_os},
	path::Path,
	str::SplitWhitespace,
};

use crate::{exitcode::ExitStatus, utils::print_flush};

pub fn cd(path: String) -> ExitStatus {
	let Some(home_dir_string) = var_os("HOME") else {
		return ExitStatus::Failed(0);
	};
	let home_dir_path = Path::new(&home_dir_string);

	if path.is_empty() {
		let res = set_current_dir(home_dir_path);
		if res.is_err() {
			print_flush(&format!("{}", res.err().unwrap()));
			return ExitStatus::Failed(2);
		}
	} else {
	}
	print_flush("\r\n");
	ExitStatus::Success(0)
}
