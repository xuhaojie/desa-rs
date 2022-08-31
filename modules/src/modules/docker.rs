use crate::{Module , BasicAction,BasicActionManager};
use clap::{Arg, ArgMatches, Command};
struct DockerModule{
	action_manager: BasicActionManager<Self>,
}

impl Module for DockerModule{

	fn name(&self) -> &'static str{
		"docker"
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
			return self.action_manager.execute_action(action, self, param);
		};
		Ok(())
	}
}

pub fn new() -> Box<dyn Module> {
	Box::new(DockerModule{
		action_manager: BasicActionManager{
			actions:vec![
				BasicAction{name:"test",  execute: action_test},
				BasicAction{name:"setup", execute: action_setup},
			]
		}
	})
}

fn action_test(module: &DockerModule, param:&ArgMatches) -> std::io::Result<()>{
	println!("test action in {}", module.name());
	Ok(())
}

fn action_setup(module: &DockerModule, param:&ArgMatches) -> std::io::Result<()>{
	println!("setup action in {}", module.name());
	if let Some(action) = param.value_of("proxy"){
		let config = param.value_of("proxy").unwrap_or("default.conf");
		println!("Value for proxy: {}", config);
	
	}
	Ok(())
}
