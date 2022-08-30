use crate::{Module , BasicAction, BasicActionManager};
use clap::{Arg, ArgMatches, Command};
use utility::{download::*, execute::*};

struct VscodeModule{
	action_manager: BasicActionManager<Self>,
}

impl Module for VscodeModule{

	fn name(&self) -> &'static str{
		"vscode"
	}

	fn command<'a>(&self) -> Command<'a> {
		Command::new(self.name())
		.about("setup vscode")
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
	let module = VscodeModule{
		action_manager: BasicActionManager{
			actions:vec![
				BasicAction{name:"download",  execute: action_download},
				BasicAction{name:"setup", execute: action_setup},
			]
		}
	};
	Box::new(module)
}

fn action_download(module: &VscodeModule, param:&ArgMatches){
	println!("test action in {}", module.name());
	match download("http://httpbin.org/get"){
		Ok(body) =>{
			println!("{}", String::from_utf8_lossy(&body));
		},
		Err(e) =>{

		},
	}
}

fn action_setup(module: &VscodeModule, param:&ArgMatches){
	println!("setup action in {}", module.name());
	if let Some(action) = param.value_of("proxy"){
		let config = param.value_of("proxy").unwrap_or("default.conf");
		println!("Value for proxy: {}", config);
	}
}

/*
#[cfg(target_os = "macos")]
static DEFAULT_PATH: &str = "path2";
#[cfg(target_os = "linux")]
static DEFAULT_PATH: &str = "path0";
#[cfg(target_os = "windows")]
static DEFAULT_PATH: &str = "path1";

if cfg!(windows) {
    println!("this is windows");
} else if cfg!(unix) {
    println!("this is unix alike");
}
*/
