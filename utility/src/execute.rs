use std::process::Command;
pub struct Cmd<'a> {
	pub cmd: &'a str,
	pub params: &'a[&'a str],
}
//pub fn execute_command(cmd: &str, params: &[&str]){
pub fn execute_command(cmd: &Cmd){
/*	
	let output = if cfg!(target_os = "windows") {
		Command::new("cmd").arg("/c").arg(cmd).output().expect("cmd exec error!")
	} else {
		Command::new("sh").arg("-c").arg(cmd).output().expect("sh exec error!")
	};
*/
	let output = Command::new(cmd.cmd).args(cmd.params).output().expect("cmd exec error!");
	let output_str = String::from_utf8_lossy(&output.stdout);
	println!("{}", output_str);
}
