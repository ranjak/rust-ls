use std::path;

pub fn list_directory_entries(dir: &path::Path) -> std::io::Result<()> {
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

