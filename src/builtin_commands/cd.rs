use std::{
	env::{set_current_dir, var_os},
	io::Error,
	path::Path,
};

use crate::{exitcode::ExitStatus, utils::print_flush};

pub fn cd(path: &str) -> ExitStatus {
	let Some(home_dir_string) = var_os("HOME") else {
		return ExitStatus::Failed(1);
	};
	let home_dir_path = Path::new(&home_dir_string);
	let Some(pwd_string) = var_os("PWD") else {
		return ExitStatus::Failed(1);
	};
	let pwd_path = Path::new(&pwd_string);
	let res: Result<(), Error>;

	if path.is_empty() {
		res = set_current_dir(home_dir_path);
	} else if path.starts_with('~') {
		let path = path.replace('~', "");
		let path_string = format!("{}{}", home_dir_path.to_str().unwrap(), path);
		let path_final = Path::new(&path_string);
		res = set_current_dir(path_final);
	} else if path.starts_with('/') {
		let path_final = Path::new(&path);
		res = set_current_dir(path_final);
	} else {
		let path_string = format!("{}/{}", pwd_path.to_str().unwrap(), path);
		let path_final = Path::new(&path_string);
		res = set_current_dir(path_final);
	}
	if res.is_err() {
		print_flush(&format!("{}", res.err().unwrap()));
		return ExitStatus::Failed(2);
	}
	print_flush("\r\n");
	ExitStatus::Success(0)
}
