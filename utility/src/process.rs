use std::process::Command;

pub fn execute_command(cmd: &str){
	
	let output = if cfg!(target_os = "windows") {
		Command::new("cmd").arg("/c").arg(cmd).output().expect("cmd exec error!")
	} else {
		Command::new("sh").arg("-c").arg(cmd).output().expect("sh exec error!")
	};

	let output_str = String::from_utf8_lossy(&output.stdout);
	println!("{}", output_str);
}

pub fn execute_commands(cmds: Vec<&str>){
	for cmd in &cmds {
		execute_command(cmd);
	}
}