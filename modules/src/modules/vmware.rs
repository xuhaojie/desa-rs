use crate::{Module , BasicAction, BasicActionManager};
use clap::{Arg, ArgMatches, Command};
use utility::download::*;
use std::path::Path;
use std::fs::{File, self};
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
	let module = VMWareModule{
		action_manager: BasicActionManager{
			actions:vec![
				BasicAction{name:"download",  execute: action_download},
			]
		}
	};
	Box::new(module)
}

fn action_download(module: &VMWareModule, param:&ArgMatches){
	println!("download action in {}", module.name());
	if cfg!(windows) {
		println!("this is windows");
	} else if cfg!(unix) {
		println!("this is unix alike");
	} else if cfg!(macos) {
		println!("this is macos");
	}

	let	url_windows = "https://www.vmware.com/go/getworkstation-win";
	let url_linux = "https://www.vmware.com/go/getworkstation-linux";
	let url_mac = "https://www.vmware.com/go/getfusion!!!";
	let url = url_mac;

	match  utility::download::get_final_url(url) {
		Ok(url) => {
			println!("get target url: {}", url);
			if let Ok(file_name) = file_name_from_url(&url) {
				println!("get filename: {}", file_name);
				let over_write = true;
				let target_folder = std::path::Path::new("/tmp");
				let target_file = Path::new("/tmp").join(file_name);
				if target_file.exists(){
					if over_write {
						fs::remove_file(target_file.as_path());
					} else {
						println!("file exists");
						return
					}
				}
					//download_file(target_url.as_str(), path.as_path());
				download_file(url.as_str(), target_folder);		
		
			} else {
				println!("can't get file from url {}", url);
				return
			}
		},
		Err(e) => {
			println!("{}", e.to_string())
		},
	}
	return
}
