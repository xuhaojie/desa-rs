use crate::{Module , BasicAction,BasicActionManager};
use clap::{Arg, ArgMatches, Command};
use utility::execute::*;
use std::io;

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
		.arg(Arg::new("mirror")
			.short('m')
			.long("mirror")
			.help("Set mirror name")
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
	let module = NpmModule{
		action_manager: BasicActionManager{
			actions:vec![
				BasicAction{name:"proxy", execute: action_setup_proxy},
			]
		}
	};
	Box::new(module)
}

fn action_setup_proxy(module: &NpmModule, param:&ArgMatches) -> std::io::Result<()>{
	let mirros = ["origin","taobao"];
	if let Some(mirror) = param.value_of("mirror"){
		
		let mut target = -1;
		let mut index = 0;
		for m in mirros.iter() {
			if *m == mirror {
				target = index;
				break;
			}
			index += 1;
		}

		if target < 0 {
			return Err(io::Error::new(io::ErrorKind::Other,"invalid mirror"));
		} else {
			let url = match mirror {
				"origin" => "https://registry.npmjs.org/",
				"taobao" => "https://registry.npm.taobao.org",
				_ => "https://registry.npmjs.org/",
			};

			let cmd = Cmd{cmd:"npm", params: vec!["config", "set", "registry", url]};
			if let Ok(code) = execute_command(&cmd) {
				if 0 == code {
					
				}
			}
			println!("set proxy to {} succeeded", mirror);
			Ok(())
		}
	} else {
		Err(io::Error::new(io::ErrorKind::Other, "miss param for mirror"))
	}
	
}