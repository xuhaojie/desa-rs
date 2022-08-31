use std::process::Command;
use std::io::{self, Write};
pub struct Cmd<'a> {
	pub cmd: &'a str,
	pub params: Vec<&'a str>,
}

impl <'a>std::string::ToString for Cmd<'a> {

	fn to_string(&self) -> String{
		format!("{} {:?}", self.cmd, self.params).to_owned()
	}
}

//pub fn execute_command(cmd: &str, params: &[&str]){
pub fn execute_command(cmd: &Cmd) -> std::io::Result<i32>{
/*	
	let output = if cfg!(target_os = "windows") {
		Command::new("cmd").arg("/c").arg(cmd).output().expect("cmd exec error!")
	} else {
		Command::new("sh").arg("-c").arg(cmd).output().expect("sh exec error!")
	};
*/

	let output = Command::new(cmd.cmd).args(&cmd.params).output().expect("cmd exec error!");
	io::stdout().write_all(&output.stdout).unwrap();
	io::stderr().write_all(&output.stderr).unwrap();
	let output_str = String::from_utf8_lossy(&output.stdout);
//	println!("status: {}", output.status);
//	println!("{}", output_str);
	match output.status.code() {
		Some(code) => Ok(code),
		None => Err(io::Error::new(io::ErrorKind::Other,format!("exec {} failed", cmd.cmd))),
	}


}
