use crate::{Module , BasicAction,BasicActionManager};
use clap::{Arg, ArgMatches, Command};
use dirs;
use std::io::{self, prelude::*,BufWriter};
use utility::{execute::*, clean::*};

struct CargoModule{
	action_manager: BasicActionManager<Self>,
}

impl Module for CargoModule{

	fn name(&self) -> &'static str{
		"go"
	}

	fn command<'a>(&self) -> Command<'a> {
		Command::new(self.name())
		.about("setup go")
		.arg(Arg::new("action")
			.help("Set the action to perform")
			.required(true))
		.arg(Arg::new("mirror")
			.short('m')
			.long("mirror")
			.help("Set mirror name")
			.takes_value(true))
		.arg(Arg::new("path")
			.short('p')
			.long("path")
			.help("Set start path")
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
	Box::new(CargoModule{
		action_manager: BasicActionManager{
			actions:vec![
				BasicAction{name:"clean",  execute: action_clean},
				BasicAction{name:"setup", execute: action_setup},
			]
		}
	})
}

fn action_clean(module: &CargoModule, param:&ArgMatches)  -> std::io::Result<()>{

	let path = match param.value_of("path"){
		Some(p) => p.to_owned(),
		None => return Err(io::Error::new(io::ErrorKind::Other,"please specify a path")),
	};

	let mut projects = Vec::<String>::new();
	search_projects(&path, "go.mod", &mut projects)?;

	for project in projects.iter() {

		let mut clean_cmd = std::process::Command::new("go");
		
		clean_cmd.current_dir(project);

		let status = clean_cmd.arg("clean").status().expect("cmd exec error!");

		match status.code() {
			Some(0) => println!("clean {} succeed", project),
			_ => println!("clean {} failed", project),
		};
	}
	Ok(())
}

fn action_setup(module: &CargoModule, param:&ArgMatches) -> std::io::Result<()>{
	println!("setup action in {}", module.name());

	let mut lines = vec![
		"[source.crates-io]\n",
		"registry =\"https://github.com/rust-lang/crates.io-index\"\n",
		"# 指定镜像\n",
		"replace-with = '镜像源名'\n",
		"# 中国科学技术大学\n",
		"[source.ustc]\n",
		"registry = \"https://mirrors.ustc.edu.cn/crates.io-index\"\n\n",
		"# 上海交通大学\n",
		"[source.sjtu]\n",
		"registry = \"https://mirrors.sjtug.sjtu.edu.cn/git/crates.io-index\"\n\n",
		"# 清华大学\n",
		"[source.tuna]\n",
		"registry = \"https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git\"\n\n",
		"# rustcc社区\n",
		"[source.rustcc]\n",
		"registry = \"https://code.aliyun.com/rustcc/crates.io-index.git\"\n\n",
	];

	//# 如：
	let mirros = ["tuna", "sjtu", "ustc", "rustcc"];

	if let Some(mirror) = param.value_of("mirror"){
		let mut find = false;
		for m in mirros.iter() {
			if *m == mirror {
				find = true;
				break;
			}
		};		
		if find {
			let set = format!("replace-with = \"{}\"\n", mirror).to_string();
			lines[3] = &set;

			let home_dir = match dirs::home_dir() {
				Some(path) => path,
				None => return Err(io::Error::new(io::ErrorKind::Other,"can't get home dir")),
			};
		
			let target_path = home_dir.join(".cargo");
			let target_file = target_path.join("config");
			let backup_file = target_path.join("config.bak");
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
		} else {
			return Err(io::Error::new(io::ErrorKind::Other,"invalid mirror"));
		};
		
	}

	Ok(())
}
