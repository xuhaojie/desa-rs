use crate::{Module};
use clap::{Arg, Command, ArgMatches};

pub struct NpmModule;

impl NpmModule{
	pub fn new() -> Self {
		NpmModule{}
	}
}

impl Module for NpmModule{
	fn cmd(&self) -> &'static str{
		"npm"
	}

	fn register<'a>(&self, cmd : Command<'a>) -> Command<'a>{
		cmd.subcommand(Command::new(self.cmd())
		.about("install or setup docker")
		.arg(Arg::new("action")
			.help("Sets the input file to use"))
		.arg(Arg::with_name("debug")
			.short('d')
			.help("print debug information verbosely")))
	}

	fn execute(&self, param: &ArgMatches) -> std::io::Result<()> {
		println!("{} execute!", self.cmd());
		if let Some(input) = param.value_of("action"){
			println!("action: {}", input);
		}
		Ok(())
	}
}
