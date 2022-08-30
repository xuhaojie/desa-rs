use crate::{Module , BasicAction,BasicActionManager};
use clap::{Arg, ArgMatches, Command};
use utility::process;
struct GitModule{
	action_manager: BasicActionManager<Self>,
}

impl Module for GitModule{

	fn name(&self) -> &'static str{
		"git"
	}

	fn command<'a>(&self) -> Command<'a> {
		Command::new(self.name())
		.about("setup cargo")
		.arg(Arg::new("action")
			.help("Sets the action to perform")
			.required(true))
		.arg(Arg::new("user")
			.short('u')
			.long("user")
			.help("Sets user name")
			.takes_value(true))			
		.arg(Arg::new("email")
			.short('e')
			.long("email")
			.help("Sets email address")
			.takes_value(true))	
	}

	fn execute(&self, param: &ArgMatches) -> std::io::Result<()> {
		if let Some(action) = param.value_of("action"){
			self.action_manager.execute_action(action, self, param);
		};
		Ok(())
	}
}

pub fn new() -> Box<dyn Module> {
	Box::new(GitModule{
		action_manager: BasicActionManager{
			actions:vec![
				BasicAction{name:"test",  execute: action_test},
				BasicAction{name:"setup", execute: action_setup},
			]
		}
	})
}

fn action_test(module: &GitModule, param:&ArgMatches){
	println!("test action in {}", module.name());
}

fn action_setup(module: &GitModule, param:&ArgMatches){
	println!("setup action in {}", module.name());

	if let Some(user) = param.value_of("user"){
		println!("user {}", user);
		let cmd = "git config --global user.name xuhaojie";
		process::execute_command(cmd);
	}

	if let Some(email) = param.value_of("email"){
		println!("email {}", email);
		let cmd = "git config --global user.email xuhaojie@hotmail.com";
		process::execute_command(cmd);

	}	
	
}
