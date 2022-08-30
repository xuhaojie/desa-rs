use crate::{Module , BasicAction,BasicActionManager};
use clap::{Arg, ArgMatches, Command};
struct NpmModule{
	action_manager: BasicActionManager<Self>,
}

impl Module for NpmModule{

	fn name(&self) -> &'static str{
		"npm"
	}

	fn command<'a>(&self) -> Command<'a> {
		Command::new(self.name())
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
			.help("print debug information verbosely"))
	}

	fn execute(&self, param: &ArgMatches) -> std::io::Result<()> {
		if let Some(action) = param.value_of("action"){
			self.action_manager.execute_action(action, self, param);
		};
		Ok(())
	}
}

pub fn new() -> Box<dyn Module> {
	let module = NpmModule{
		action_manager: BasicActionManager{
			actions:vec![
				BasicAction{name:"test",  execute: action_test},
				BasicAction{name:"setup", execute: action_setup},
			]
		}
	};
	Box::new(module)
}

fn action_test(module: &NpmModule, param:&ArgMatches){
	println!("test action in {}", module.name());
}

fn action_setup(module: &NpmModule, param:&ArgMatches){
	println!("setup action in {}", module.name());
}
