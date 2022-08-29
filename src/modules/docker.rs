use crate::Module;
use clap::{Arg, App, SubCommand, ArgMatches};
pub fn new() -> Box<dyn Module>{
	Box::new(DockerModule{})
}

struct DockerModule{

}

impl <'a>Module for DockerModule {
	fn cmd(&self) -> &'static str{
		"docker"
	}
	fn register(&self, app : App) -> App{
		app.subcommand(SubCommand::with_name(self.cmd())
		.about("install or setup docker")
		.version("1.3")
		.author("Someone E. <someone_else@other.com>")
		.arg(Arg::with_name("debug")
			.short('d')
			.help("print debug information verbosely"))).clone()
	}
	fn execute(&self, param: &ArgMatches){

	}
}
