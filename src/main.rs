use std::env;
use std::path;
use std::io;
use std::error::Error;
mod rust_ls;


fn main() {
	run().unwrap_or_else(|e| {
		eprintln!("{}", e);
		std::process::exit(1);
	});
}

fn run() -> std::result::Result<(), String> {
	// Accept a directory argument from command line, or use the working directory.
	let dir_to_list = if env::args().count() == 2 {
		get_path_from_arg(&env::args().nth(1).unwrap())
	}
	else {
		env::current_dir()
	};
	

	let dir_to_list = dir_to_list.map_err(|e| {
		format!("Error: No valid directory to list: {}", e.description())
	})?;
	
	println!("Directory to list: {}", dir_to_list.display());
	
	rust_ls::list_directory_entries(dir_to_list.as_path()).map_err(|e| {
		format!("Error listing directory entries: {}", e.description())
	})?;
	
	Ok(())
}	

// Make sure the argument is a valid path
fn get_path_from_arg(arg: &String) -> std::io::Result<path::PathBuf> {
	let path = path::PathBuf::from(arg);
	let mut target_dir = env::current_dir()?;
	target_dir.push(path);
	target_dir = target_dir.canonicalize()?;
	if !target_dir.is_dir() {
		Err(io::Error::new(io::ErrorKind::Other, format!("The provided path ({}) is not a directory", arg)))
	}
	else {
		Ok(target_dir)
	}
}

