use crate::{Module , BasicAction, BasicActionManager};
use clap::{Arg, ArgMatches, Command};
use utility::{platform::*,download::*};
use std::io;
struct VMWareModule{
	action_manager: BasicActionManager<Self>,
}

impl Module for VMWareModule{

	fn name(&self) -> &'static str{
		"vmware"
	}

	fn command<'a>(&self) -> Command<'a> {
		Command::new(self.name())
		.about("download vmware")
		.arg(Arg::new("action")
			.help("Sets the action to perform")
			.required(true))
		.arg(Arg::new("proxy")
			.short('p')
			.long("proxy")
			.help("Sets a custom proxy")
			.takes_value(true))
		.arg(Arg::new("os")
			.short('o')
			.long("os")
			.help("os type")
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
	let module = VMWareModule{
		action_manager: BasicActionManager{
			actions:vec![
				BasicAction{name:"download",  execute: action_download},
			]
		}
	};
	Box::new(module)
}

fn action_download(module: &VMWareModule, param:&ArgMatches) -> std::io::Result<()> {
	println!("download action in {}", module.name());

	let os = match param.value_of("os") {
		Some(os) => Platform::from(os),
		None => current_platform(),
	};

	let url = match os {
		Platform::WINDOWS => "https://www.vmware.com/go/getworkstation-win",
		Platform::LINUX => "https://www.vmware.com/go/getworkstation-linux",
		Platform::MACOS => "https://www.vmware.com/go/getfusion",
		_ => {
			return Err(io::Error::new(io::ErrorKind::Other,format!("not support os type {}", os)));
		},
	};
	let target_url = utility::download::get_redirected_url(url)?;
	println!("get target url: {}", target_url);
	let target_folder = std::path::Path::new("/tmp");
	download_file(target_url.as_str(), target_folder, true)
}
