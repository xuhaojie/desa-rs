use std::{io, ops::Index};
use crate::{Module , BasicAction, BasicActionManager};
use clap::{Arg, ArgMatches, Command};
use utility::{download::*, execute::*, platform::*, arch::*, package::*};
use std::fmt;

struct NomachineModule{
	action_manager: BasicActionManager<Self>,
}

impl Module for NomachineModule{

	fn name(&self) -> &'static str{
		"nomachine"
	}

	fn command<'a>(&self) -> Command<'a> {
		Command::new(self.name())
		.about("setup nomachine")
		.arg(Arg::new("action")
			.help("Sets the action to perform")
			.required(true))
		.arg(Arg::new("os")
			.short('o')
			.long("os")
			.help("os type")
			.takes_value(true))
		.arg(Arg::new("arch")
			.short('a')
			.long("arch")
			.help("arch type")
			.takes_value(true))			
		.arg(Arg::new("package")
			.short('k')
			.long("package")
			.help("package type")
			.takes_value(true))
		.arg(Arg::new("folder")
			.short('f')
			.long("folder")
			.help("target folder")
			.takes_value(true))						
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
	let module = NomachineModule{
		action_manager: BasicActionManager{
			actions:vec![
				BasicAction{name:"download", execute: action_download},
			]
		}
	};
	Box::new(module)
}

fn action_download(module: &NomachineModule, param:&ArgMatches) -> std::io::Result<()> {
	println!("download action in {}", module.name());

	let platform = match param.value_of("os") {
		Some(os) => Platform::from(os),
		None => current_platform(),
	};

	let arch = match param.value_of("arch") {
		Some(a) => Arch::from(a),
		None => current_arch(),
	};

	let folder = match param.value_of("folder") {
		Some(f) => f.to_string(),
		None => std::env::current_dir().unwrap().to_str().unwrap().to_owned(),
	};

	let pkg = match param.value_of("package") {
		Some(pkg_type) => PackageType::from(pkg_type),
		None => {
			match platform {
				Platform::LINUX => PackageType::DEB,
				Platform::WINDOWS => PackageType::EXE,
				_ => PackageType::UNKNOWN,
			}
		}
	};

	//https://www.nomachine.com/download/linux&id=29&s=Raspberry // https://www.nomachine.com/download/download&id=106&s=Raspberry&hw=Pi2
	let download_id = match platform {

		Platform::LINUX => match arch {
			Arch::X86_64 =>	match pkg {
				PackageType::RPM => 1, //url := "https://www.nomachine.com/download/download&id=1" // https://download.nomachine.com/download/7.9/Linux/nomachine_7.9.2_1_x86_64.rpm
				PackageType::ARCHIVE => 2, //url := "https://www.nomachine.com/download/download&id=2" // https://download.nomachine.com/download/7.9/Linux/nomachine_7.9.2_1_x86_64.tar.gz
				PackageType::DEB => 4, //url := //url := "https://www.nomachine.com/download/download&id=4" // https://download.nomachine.com/download/7.9/Linux/nomachine_7.9.2_1_amd64.deb
				_ => {
					return Err(io::Error::new(io::ErrorKind::Other,format!("pkg {} not supported on {} platform", pkg ,platform)));
				},
			},
			Arch::X86 => match pkg {
				PackageType::ARCHIVE => 3, //url := "https://www.nomachine.com/download/download&id=3" // https://download.nomachine.com/download/7.9/Linux/nomachine_7.9.2_1_i686.tar.gz
				PackageType::RPM => 5, //url := "https://www.nomachine.com/download/download&id=5" // https://download.nomachine.com/download/7.9/Linux/nomachine_7.9.2_1_i686.rpm
				PackageType::DEB => 6, //url := "https://www.nomachine.com/download/download&id=6" // https://download.nomachine.com/download/7.9/Linux/nomachine_7.9.2_1_i386.deb
				_ => {
					return Err(io::Error::new(io::ErrorKind::Other,format!("pkg {} not supported on {} platform", pkg ,platform)));
				},				
			},
			_ => {
				return Err(io::Error::new(io::ErrorKind::Other,format!("arch not supported on {} platform{}", platform, arch.to_string())));
			},
		},
		Platform::MACOS => 7, //url = "https://www.nomachine.com/download/download&id=7" // https://download.nomachine.com/download/7.9/MacOSX/nomachine_7.9.2_1.dmg
		Platform::WINDOWS => 8 , //"https://www.nomachine.com/download/download&id=8" // https://download.nomachine.com/download/7.9/Windows/nomachine_7.9.2_1.exe
		_ => {
			return Err(io::Error::new(io::ErrorKind::Other,format!("os {} not supported", platform)));
		}
	};

	let url = format!("https://www.nomachine.com/download/download&id={}", download_id);
	println!("url: {}", url);

	let response = reqwest::blocking::get(url);
	let content = match response {
		Ok(r) => {
			if let Ok(body) = r.text() {
				//println!("body = {:?}", body);
				body

			} else {
				return Err(io::Error::new(io::ErrorKind::Other,format!("os {} not supported", platform)));
			}
		},
		_ => return Err(io::Error::new(io::ErrorKind::Other,format!("os {} not supported", platform))),
	};

	let target_str = "'https://download.nomachine.com/download/";

	let start = if let Some(index) = content.find(target_str){
		println!("start:{}",index);
		index
	} else {
		return Err(io::Error::new(io::ErrorKind::Other,format!("os {} not supported", platform)));
	};

	let end = if let Some(index) = content[start..].find("');\""){
		index
	} else {
		return Err(io::Error::new(io::ErrorKind::Other,format!("os {} not supported", platform)));
	};

	let target_url = content[start + 1..start+end].to_string();
	println!("target url: {}", target_url);

	let target_folder = std::path::Path::new(&folder);
	download_file(&target_url, target_folder, true)
}
