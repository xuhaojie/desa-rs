use crate::{Module , BasicAction, BasicActionManager};
use clap::{Arg, ArgMatches, Command};
use dirs;
use std::io::{self, prelude::*,BufWriter};
use utility::{clean::*, platform::*};
use reqwest::Url;
struct PipModule{
	action_manager: BasicActionManager<Self>,
}

impl Module for PipModule{

	fn name(&self) -> &'static str{
		"pip"
	}

	fn command<'a>(&self) -> Command<'a> {
		Command::new(self.name())
		.about("setup pip")
		.arg(Arg::new("action")
			.help("Set the action to perform")
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
	Box::new(PipModule{
		action_manager: BasicActionManager{
			actions:vec![
				BasicAction{name:"proxy", execute: action_setup_proxy},
			]
		}
	})
}

fn action_setup_proxy(module: &PipModule, param:&ArgMatches) -> std::io::Result<()>{

	let mut lines = vec![
		"[global]\n",
		"index-url=https://pypi.tuna.tsinghua.edu.cn/simple\n",
		"[install]\n",
		"trusted-host=https://pypi.tuna.tsinghua.edu.cn\n",
	];

	//# 如：
	let mirros = ["tuna", "163", "aliyun"];

	if let Some(mirror) = param.value_of("mirror"){
		let mut find = false;
		for m in mirros.iter() {
			if *m == mirror {
				find = true;
				break;
			}
		};		
		if find {
	
			let url = match mirror {
				"tuna" => "https://pypi.tuna.tsinghua.edu.cn/simple",
				"163" => "https://mirrors.163.com/pypi/simple",
				"aliyun" => "http://mirrors.aliyun.com/pypi/simple",
				_ => "https://pypi.tuna.tsinghua.edu.cn/simple",
			};
			let url = Url::parse(&url).unwrap();

			
			let set = format!("index-url={}\n", url.to_string());
			lines[1] = &set;

			let set = format!("trusted-host={}\n", url.host().unwrap().to_string());
			lines[3] = &set;

			let home_dir = match dirs::home_dir() {
				Some(path) => path,
				None => return Err(io::Error::new(io::ErrorKind::Other,"can't get home dir")),
			};
		

			let (folder_name,file_name, backup_file) = match current_platform() {
				Platform::LINUX | Platform::MACOS  => (".pip","pip.conf","pip.conf.bak"),
				Platform::WINDOWS => ("pip","pip.ini","pip.ini.bak"),
				_ =>  return Err(io::Error::new(io::ErrorKind::Other,"unsupported platform")),
			};

			let target_path = home_dir.join(folder_name);
			let target_file = target_path.join(file_name);
			let backup_file = target_path.join(backup_file);
			if !target_path.exists(){
				std::fs::create_dir(target_path);
			}
			if target_file.exists(){
				if backup_file.exists(){
					std::fs::remove_file(backup_file.as_path());
				}
				std::fs::rename(target_file.as_path(), backup_file.as_path());
			}

			let mut buffer = BufWriter::new(std::fs::File::create(target_file)?);
			for line in lines.iter() {
				buffer.write_all(line.as_bytes())?;
			}
			buffer.flush()?;
			println!("set proxy to {} succeeded", mirror);
			Ok(())
		} else {
			Err(io::Error::new(io::ErrorKind::Other,"invalid mirror"))
		}
		
	} else {
		Err(io::Error::new(io::ErrorKind::Other, "miss param for mirror"))
	}


}
