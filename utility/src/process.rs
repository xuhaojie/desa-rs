use std::process::Command;

pub fn execute_command(cmd: &str){
	// cmd_str可以是从输入流读取或从文件里读取
	let cmd_str = if cfg!(target_os = "windows") {
		// 这里不用\\而是/的话会被windows认为/tmp的/t是一个option而报错
		"dir d:\\tmp".to_string()
	} else {
		"ls /Users".to_string()
	};

	let output = if cfg!(target_os = "windows") {
		Command::new("cmd").arg("/c").arg(cmd_str).output().expect("cmd exec error!")
	} else {
		Command::new("sh").arg("-c").arg(cmd_str).output().expect("sh exec error!")
	};

	let output_str = String::from_utf8_lossy(&output.stdout);
	println!("{}", output_str);
}