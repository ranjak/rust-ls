use std::env;
use std::path;
use std::io;
use std::error::Error;

fn main() {
	// Accept a directory argument from command line, or use the working directory.
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
	
	list_directory_entries(dir_to_list.as_path()).unwrap_or_else(|e| {
		eprintln!("Error listing directory entries: {}", e.description());
		std::process::exit(1);
	});
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

fn list_directory_entries(dir: &path::Path) -> std::io::Result<()> {
	for entry in dir.read_dir()? {
		let entry = entry?;
		let entry_type = match entry.file_type()? {
			t if t.is_dir() => "Directory",
			t if t.is_file() => "File",
			t if t.is_symlink() => "Symlink",
			_ => "Unknown"
		};
		let entry_name = entry.file_name().into_string().unwrap_or_else(|_| {
			String::from("<Error: name contains invalid unicode data>")
		});
		
		println!("[{}] {}", entry_type, entry_name);
	}
	Ok(())
}
