use crate::Module;
use clap::{Arg, ArgMatches, Command};
use utility::{download::*, process::*};


pub struct VscodeModule;

impl VscodeModule{
	pub fn new() -> Self {
		VscodeModule{}
	}
}


/*
#[cfg(target_os = "macos")]
static DEFAULT_PATH: &str = "path2";
#[cfg(target_os = "linux")]
static DEFAULT_PATH: &str = "path0";
#[cfg(target_os = "windows")]
static DEFAULT_PATH: &str = "path1";

if cfg!(windows) {
    println!("this is windows");
} else if cfg!(unix) {
    println!("this is unix alike");
}
*/

impl Module for VscodeModule{
	fn name(&self) -> &'static str{
		"vscode"
	}
	fn command<'a>(&self) -> Command<'a> {
		Command::new(self.name())
		.about("setup vscode")
		.arg(Arg::new("action")
			.help("Sets the action to perform")
			.required(true))
		.arg(Arg::new("proxy")
			.short('p')
			.long("proxy")
			.help("Sets a custom proxy")
			.takes_value(true))			
		.arg(Arg::new("debug")
			.short('d')
			.help("print debug information verbosely"))
	}

	fn execute(&self, param: &ArgMatches) -> std::io::Result<()>{
		if let Some(action) = param.value_of("action"){
			match action{
				"download" => {
					println!("download");
					match download("http://httpbin.org/get"){
						Ok(body) =>{
							println!("{}", String::from_utf8_lossy(&body));
						},
						Err(e) =>{

						},
					}
				},
				"setup" => {
					println!("setup");
					execute_command("");
					if let Some(action) = param.value_of("proxy"){
						let config = param.value_of("proxy").unwrap_or("default.conf");
						println!("Value for proxy: {}", config);
					}
				},
				_ => println!("unkonwn action: {}", action),
			}
		};
		Ok(())
	}
}