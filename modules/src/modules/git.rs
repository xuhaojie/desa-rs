use crate::Module;
use clap::{Arg, ArgMatches, Command};
pub struct GitModule;

impl GitModule{
	pub fn new() -> Self {
		GitModule{}
	}
}

impl Module for GitModule{
	fn name(&self) -> &'static str{
		"git"
	}
	fn command<'a>(&self) -> Command<'a> {
		Command::new(self.name())
		.about("setup git")
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
	/*
	fn register<'a>(&self, cmd : Command<'a>) -> Command<'a>{
		cmd.subcommand(self.command().clone())
	}
	*/
	fn execute(&self, param: &ArgMatches) -> std::io::Result<()> {
		
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
