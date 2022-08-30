use crate::Module;
use clap::{Arg, ArgMatches, Command};
pub struct AptModule;

impl AptModule{
	pub fn new() -> Self {
		AptModule{}
	}
}

impl Module for AptModule{
	fn cmd(&self) -> &'static str{
		"apt"
	}
	fn register<'a>(&self, cmd : Command<'a>) -> Command<'a>{
		cmd.subcommand(Command::new(self.cmd())
		.about("setup apt")
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
	fn execute(&self, param: &ArgMatches) -> std::io::Result<()> {
		println!("{} execute!", self.cmd());
		if let Some(action) = param.value_of("action"){
			match action{
				"install" => {
					println!("install");


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
		Ok(())
	}
}
