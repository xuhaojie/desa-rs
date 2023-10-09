use std::{io::{self, prelude::*, BufWriter}, path::Path};

pub fn write_data_to_file(data: &[u8],target_path: &Path, file_name: &str, backup_file:&str) -> std::io::Result<()>{

	let target_file = target_path.join(file_name);

	if !target_path.exists() {
		let _ = std::fs::create_dir(target_path);
	}
	if target_file.exists() {
		if backup_file.len() > 0 {
			let backup_file = target_path.join(backup_file);
			if backup_file.exists() {
				let _ = std::fs::remove_file(backup_file.as_path());
			}
			let _ = std::fs::rename(target_file.as_path(), backup_file.as_path());
		} else {
			let _ = std::fs::remove_file(target_file.as_path());
		}
	
	}

	let mut buffer = BufWriter::new(std::fs::File::create(target_file)?);
	/*
	for line in lines.iter() {
		buffer.write_all(line.as_bytes())?;
	}
	*/
	buffer.write(data)?;
	buffer.flush()?;
	Ok(())
}


pub fn write_lines_to_file(lines: &Vec<String>,target_path: &Path, file_name: &str, backup_file:&str) -> std::io::Result<()>{

	let target_file = target_path.join(file_name);
	

	if !target_path.exists() {
		let _ = std::fs::create_dir(target_path);
	}
	if target_file.exists() {
		if backup_file.len() > 0 {
			let backup_file = target_path.join(backup_file);
			if backup_file.exists() {
				let _ = std::fs::remove_file(backup_file.as_path());
			}
			let _ = std::fs::rename(target_file.as_path(), backup_file.as_path());
		} else {
			let _ = std::fs::remove_file(target_file.as_path());
		}
	
	}

	let mut buffer = BufWriter::new(std::fs::File::create(target_file)?);
	for line in lines.iter() {
		buffer.write_all(line.as_bytes())?;
	}
	buffer.flush()?;

	Ok(())
}

