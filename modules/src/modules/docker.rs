use std::path::Path;

use crate::{BaseModule, BasicAction, Module};
use clap::{Arg, ArgMatches, Command};
use utility::{file::write_data_to_file, execute::{Cmd, execute_command}};

pub fn new() -> Box<dyn Module> {
    Box::new(BaseModule {
        name: "docker",
        description: "Setup docker",
        actions: vec![BasicAction {
            name: "proxy",
            cmd: || {
                Command::new("proxy").about("setup docker proxy")
            },
            execute: action_setup,
        }],
    })
}
/*
 /etc/docker/daemon.json
 {
    "registry-mirrors": [
        "https://registry.hub.docker.com",
        "http://hub-mirror.c.163.com",
        "https://docker.mirrors.ustc.edu.cn",
        "https://registry.docker-cn.com"
    ]
}
systemctl restart docker
*/

fn action_setup(_parent: Option<&dyn Module>, param: &ArgMatches) -> std::io::Result<()> {
    if let Some(parent) = _parent {
        println!("setup action in {}", parent.name());
    }
	let cfg = b"{\n \
		\t\"registry-mirrors\": [ \n\
			\t\t\"https://registry.hub.docker.com\",\n \
			\t\t\"http://hub-mirror.c.163.com\",\n \
			\t\t\"https://docker.mirrors.ustc.edu.cn\",\n \
			\t\t\"https://registry.docker-cn.com\"\n \
		\t]\n \
	}\n";
	write_data_to_file(cfg,Path::new("/etc/docker/"),"daemon.json", "daemon.json.bak")?;
	let command = Cmd {
		cmd: "systemctl",
		params: vec!["restart", "docker"],
	};

	if let Ok(code) = execute_command(&command) {
		if 0 == code {
			println!("exec {} succeed", command.to_string());
		}
	}

    Ok(())
}
