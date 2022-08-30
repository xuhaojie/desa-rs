
pub fn execute_command(){
	use std::process;

	let output = if cfg!(target_os = "windows") {
		process::Command::new("cmd")
				.args(["/C", "echo hello12"])
				.output()
				.expect("failed to execute process")
	} else {
		process::Command::new("sh")
				.arg("-c")
				.arg("echo hello11")
				.output()
				.expect("failed to execute process")
	};


	println!("status: {}", &output.status);
	stdout().write_all(&output.stdout).unwrap();
	stderr().write_all(&output.stderr).unwrap();
}