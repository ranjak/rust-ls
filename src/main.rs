use std::env;
use std::path;
use std::io;
use std::error::Error;

fn main() {
	let dir_to_list = if env::args().count() == 2 {
		get_path_from_arg(&env::args().nth(1).unwrap())
	}
	else {
		env::current_dir()
	};
	

	let dir_to_list = dir_to_list.unwrap_or_else(|e| {
		eprintln!("Error: No valid directory to list: {}", e.description());
		std::process::exit(1);
	});
	
	println!("Directory to list: {}", dir_to_list.display());
}

// Make sure the argument is a valid path
fn get_path_from_arg(arg: &String) -> std::io::Result<path::PathBuf> {
	let path = path::PathBuf::from(arg);
	let mut target_dir = env::current_dir()?;
	target_dir.push(path);
	if !target_dir.is_dir() {
		Err(io::Error::new(io::ErrorKind::Other, format!("The provided path ({}) is not a directory", arg)))
	}
	else {
		Ok(target_dir)
	}
}
