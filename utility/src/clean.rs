pub fn search_projects(start_path: &str, mark_file: &str, projects: &mut Vec<String>) -> std::io::Result<()> {
	for entry in std::fs::read_dir(start_path)? {
		let entry = entry?;
		let path = entry.path();
	
		let metadata = std::fs::metadata(&path)?;
		if metadata.is_file() {
			if let Some(p) = path.to_str() {
				if let Some(file_name) =path.file_name() {
					if file_name == mark_file {
						projects.push(start_path.to_string());
					}
				}
			}
		}
		if metadata.is_dir() {
			if let Some(p) = path.to_str() {
				search_projects(p,mark_file, projects)?
			}
		}
	}
	Ok(())
}

pub fn clean_projects(projects: &Vec<String>, cmd: &str, params: &[&str]) -> std::io::Result<()> {
	for project in projects.iter() {

		let mut clean_cmd = std::process::Command::new(cmd);
		
		clean_cmd.current_dir(project);

		let status = clean_cmd.args(params).status().expect("cmd exec error!");

		match status.code() {
			Some(0) => println!("clean {} succeed", project),
			_ => println!("clean {} failed", project),
		};
	}
	Ok(())
}