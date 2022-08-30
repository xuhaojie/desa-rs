use crate::{Module};
use clap::{Arg, Command, ArgMatches};

pub struct NpmModule;

impl NpmModule{
	pub fn new() -> Self {
		NpmModule{}
	}
}

impl Module for NpmModule{
	fn name(&self) -> &'static str{
		"npm"
	}
	fn command<'a>(&self) -> Command<'a> {
		Command::new(self.name())
		.about("install or setup docker")
		.arg(Arg::new("action")
			.help("Sets the input file to use"))
		.arg(Arg::with_name("debug")
			.short('d')
			.help("print debug information verbosely"))
	}

	fn execute(&self, param: &ArgMatches) -> std::io::Result<()> {
		if let Some(input) = param.value_of("action"){
			println!("action: {}", input);
		}
		Ok(())
	}
}
