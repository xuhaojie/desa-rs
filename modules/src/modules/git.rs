use crate::{Module , BasicAction,BasicActionManager};
use clap::{Arg, ArgMatches, Command};
use utility::execute::{self, Cmd};


struct GitModule{
	action_manager: BasicActionManager<Self>,
}

impl Module for GitModule{

	fn name(&self) -> &'static str{
		"git"
	}

	fn command<'a>(&self) -> clap::Command<'a> {
		clap::Command::new(self.name())
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
			return self.action_manager.execute_action(action, self, param);
		};
		Ok(())
	}
}

pub fn new() -> Box<dyn Module> {
	Box::new(GitModule{
		action_manager: BasicActionManager{
			actions:vec![
				BasicAction{name:"setup", execute: action_setup},
			]
		}
	})
}

fn action_setup(module: &GitModule, param:&ArgMatches) -> std::io::Result<()>{

	if let Some(user) = param.value_of("user"){
		let mut cmd = Cmd{cmd:"git", params: vec!["config", "--global", "user.name"]};
		//let cmd = format!("git config --global user.name {}", user);
		cmd.params.push(user);
		if let Ok(code) = execute::execute_command(&cmd) {
			if 0 == code {
				println!("exec {} succeed", cmd.to_string());
			}
		}
	}

	if let Some(email) = param.value_of("email"){
		let mut cmd = Cmd{cmd:"git", params: vec!["config", "--global", "user.email"]};
		//let cmd = format!("git config --global user.name {}", user);
		cmd.params.push(email);
		if let Ok(code) = execute::execute_command(&cmd) {
			if 0 == code {
				println!("exec {} succeed", cmd.to_string());
			}
		}
	}
	Ok(())
	
}
