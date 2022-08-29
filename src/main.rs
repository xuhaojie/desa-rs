pub mod modules;
use std::io::{stderr,stdout,Write};

use crate::modules::*;

extern crate clap;
use clap::{Arg, App, SubCommand, ArgMatches, Command};
pub trait Module {
	fn cmd(&self) -> &'static str;
	fn register<'a>(&self, app : App<'a>) -> App<'a>;
	fn execute(&self, param: &ArgMatches);
}
pub struct SubModule<'a> {
	cmd: &'a str,
	title: &'static str,

}


fn main() {
	use std::process;

	let output = if cfg!(target_os = "windows") {
		process::Command::new("cmd")
				.args(["/C", "echo hello12"])
				.output()
				.expect("failed to execute process")
	} else {
		process::Command::new("sh")
				.arg("-c")
				.arg("echo hello11")
				.output()
				.expect("failed to execute process")
	};
	

	println!("status: {}", &output.status);
	stdout().write_all(&output.stdout).unwrap();
	stderr().write_all(&output.stderr).unwrap();

	let modules: Vec<Box<dyn Module>> = vec![
		Box::new(apt::AptModule::new()),
		Box::new(docker::DockerModule::new()),
		Box::new(npm::NpmModule::new()),
	];
	let mut cmd = Command::new("desa")
						  .version("1.0")
						  .author("Kevin K. <kbknapp@gmail.com>")
						  .about("Does awesome things")
						  .arg(Arg::with_name("config")
							   .short('c')
							   .long("config")
							   .value_name("FILE")
							   .help("Sets a custom config file")
							   .takes_value(true))
						  .arg(Arg::with_name("INPUT")
							   .help("Sets the input file to use")
							   .required(false)
							   .index(1))
						  .arg(Arg::with_name("ver")
							   .short('v')
							   .help("Sets the level of verbosity"))
						  .subcommand(SubCommand::with_name("test")
									  .about("controls testing features")
									  .version("1.3")
									  .author("Someone E. <someone_else@other.com>")
									  .arg(Arg::with_name("debug")
										  .short('d')
										  .help("print debug information verbosely")));

	for module in modules.iter() {
		cmd = module.register(cmd);
	}

	let matches = cmd.get_matches();

	// 如果用户提供、则获取该值作为config，或者默认使用 “default.conf”
	let config = matches.value_of("config").unwrap_or("default.conf");
	println!("Value for config: {}", config);

	// 在这里调用.unwrap（）是安全的，因为需要“ INPUT”（如果不需要“ INPUT”，
	// 可以使用 “if let” 有条件地获取值）

	if let Some(input) = matches.value_of("INPUT"){
		println!("Using input file: {}", input);
	}

	// 根据用户使用“详细”标志的次数来改变输出
	// (比如 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
	/*
	match matches.occurrences_of("v") {
		0 => println!("No verbose info"),
		1 => println!("Some verbose info"),
		2 => println!("Tons of verbose info"),
		3 | _ => println!("Don't be crazy"),
	}
	*/

	// 你可以通过以下方式处理有关子命令的信息：按名称请求它们的匹配（如下所示）
	// 仅请求正在使用的名称或两者同时请求

	for module in modules.iter() {
		if let Some(matches) = matches.subcommand_matches(module.cmd()) {
			module.execute(matches);
			break;
		}
	}

	// 其他程序逻辑...
}