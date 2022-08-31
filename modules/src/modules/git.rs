use crate::{Module , BasicAction,BasicActionManager};
use clap::{Arg, ArgMatches, Command};
use std::process::Command as ExecuteCommand;
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

fn action_test(module: &GitModule, param:&ArgMatches) -> std::io::Result<()>{
	println!("test action in {}", module.name());
	Ok(())
}

fn action_setup(module: &GitModule, param:&ArgMatches) -> std::io::Result<()>{
	println!("setup action in {}", module.name());
	
	let output = ExecuteCommand::new("ls").args(["-l","-a"]).output().ok().expect("Failed to execute.");

	use std::io::{self, Write};

	if let Some(user) = param.value_of("user"){
		println!("user {}", user);
		//let cmd = format!("git config --global user.name {}", user);
		let mut params = vec!["config", "--global", "user.name"];
		params.push(user);
		
		let output = ExecuteCommand::new("git").args(params).output().ok().expect("Failed to execute.");
		println!("status: {}", output.status.code().unwrap());
		//let cmd = Cmd{cmd:"git", params:&params};
		//execute::execute_command(&cmd );
		io::stdout().write_all(&output.stdout).unwrap();
		io::stderr().write_all(&output.stderr).unwrap();
	}

	if let Some(email) = param.value_of("email"){
		println!("email {}", email);
		//let cmd = format!("git config --global user.email {}",email);
		let mut params = vec!["config", "--global", "user.email"];
		params.push(email);		
		//let cmd = Cmd{cmd:"git", params:&params};
		let output = ExecuteCommand::new("git").args(params).output().ok().expect("Failed to execute.");
		println!("status: {}", output.status);
		io::stdout().write_all(&output.stdout).unwrap();
		io::stderr().write_all(&output.stderr).unwrap();
	}
	Ok(())
	
}
