
use crate::Module;
use clap::{Arg, ArgMatches, Command};
use utility::download::*;
pub struct VscodeModule;

impl VscodeModule{
	pub fn new() -> Self {
		VscodeModule{}
	}
}

impl Module for VscodeModule{
	fn cmd(&self) -> &'static str{
		"vscode"
	}
	fn register<'a>(&self, cmd : Command<'a>) -> Command<'a>{
		cmd.subcommand(Command::new(self.cmd())
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
			.help("print debug information verbosely")))
	}
	fn execute(&self, param: &ArgMatches){
		println!("{} execute!", self.cmd());
		if let Some(action) = param.value_of("action"){
			match action{
				"download" => {
					println!("download");
					download("");
				},
				"setup" => {
					println!("setup");
					if let Some(action) = param.value_of("proxy"){
						let config = param.value_of("proxy").unwrap_or("default.conf");
						println!("Value for proxy: {}", config);
					}
				},
				_ => println!("unkonwn action: {}", action),
			}
		};
	}
}