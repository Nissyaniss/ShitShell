use std::{
	env::{set_current_dir, set_var, var_os},
	path::Path,
};

use crate::{types::exitcode::ExitStatus, types::utils::print_flush};

pub fn cd(path: &str) -> ExitStatus {
	let Some(home_dir_osstring) = var_os("HOME") else {
		return ExitStatus::Failed(1);
	};
	let Some(home_dir) = home_dir_osstring.to_str() else {
		return ExitStatus::Failed(1);
	};
	let absolute_path_string = &path.replace('~', home_dir);
	let path_final = if path.is_empty() {
		Path::new(home_dir)
	} else {
		Path::new(absolute_path_string)
	};
	let Ok(absolute_path) = path_final.canonicalize() else {
		return ExitStatus::Failed(1);
	};
	let res = set_current_dir(absolute_path.clone());
	if res.is_err() {
		print_flush(&format!(
			"\nShitShell: cd: {path}: {}\n",
			res.err().unwrap()
		));
		return ExitStatus::Failed(2);
	}
	set_var("PWD", absolute_path.into_os_string());
	print_flush("\r\n");
	ExitStatus::Success(0)
}
