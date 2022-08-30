use crate::Module;
use clap::{Arg, ArgMatches, Command};

pub struct DockerModule;

impl DockerModule{
	pub fn new() -> Self {
		DockerModule{}
	}
}
	
impl Module for DockerModule{
	fn cmd(&self) -> &'static str{
		"docker"
	}
	fn register<'a>(&self, cmd : Command<'a>) -> Command<'a>{
		cmd.subcommand(Command::new(self.cmd())
		.about("install or setup docker")
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

	}
}
