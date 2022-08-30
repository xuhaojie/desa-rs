use crate::{Module , BasicAction, BasicActionManager};
use clap::{Arg, ArgMatches, Command};
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
	let url_mac = "https://www.vmware.com/go/getfusion";
	let url = url_mac;

	let target_url = utility::download::get_final_url(url);

	println!("get target url: {}", target_url);
	
	let fields: Vec<&str> = target_url.split('/').collect();
	if fields.len() < 2 {
		return; // Err(io::Error::new(io::ErrorKind::Other,"bad request"));
	}
	let file_name = fields[fields.len()-1];
	println!("get filename: {}", file_name);
	return
}
