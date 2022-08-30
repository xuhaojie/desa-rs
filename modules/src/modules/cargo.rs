use crate::{Module , BasicAction,BasicActionManager};
use clap::{Arg, ArgMatches, Command};
struct CargoModule{
	action_manager: BasicActionManager<Self>,
}

impl Module for CargoModule{

	fn name(&self) -> &'static str{
		"cargo"
	}

	fn command<'a>(&self) -> Command<'a> {
		Command::new(self.name())
		.about("setup cargo")
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
	Box::new(CargoModule{
		action_manager: BasicActionManager{
			actions:vec![
				BasicAction{name:"test",  execute: action_test},
				BasicAction{name:"setup", execute: action_setup},
			]
		}
	})
}

fn action_test(module: &CargoModule, param:&ArgMatches){
	println!("test action in {}", module.name());
}

fn action_setup(module: &CargoModule, param:&ArgMatches){
	println!("setup action in {}", module.name());
	if let Some(action) = param.value_of("proxy"){
		let config = param.value_of("proxy").unwrap_or("default.conf");
		println!("Value for proxy: {}", config);
	
	}
}
