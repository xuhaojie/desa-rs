use crate::{BaseModule, BasicAction, Module};
use anyhow::anyhow;
use clap::{Arg, ArgMatches, Command};
use std::path::Path;
use utility::{
    execute::{execute_command, Cmd},
    file::write_lines_to_file,
	registry::{self,Registry, list_registers, set_registry},
};

pub fn new() -> Box<dyn Module> {
    Box::new(BaseModule {
        name: "docker",
        description: "Setup docker",
        actions: vec![BasicAction {
            name: "mirror",
            cmd: || Command::new("mirror")
			.about("setup docker mirror")
			.arg(
				Arg::new("mirror")
					//.short('m')
					//.long("mirror")
					.help("mirror name, use -l to list")
					.takes_value(true),
			)
			.arg(
				Arg::new("list")
					.short('l')
					.long("list")
					.help("list available cargo registers")
					.action(clap::ArgAction::SetTrue),
			),
            execute: action_setup_mirror,
        },
		BasicAction {
            name: "user",
            cmd: || Command::new("user")
			.about("setup docker user")
			.arg(
				Arg::new("user")
					.help("user name")
					.takes_value(true),
			),
            execute: action_setup_user,
        }],
    })
}

static REGISTRYS:[Registry;5] = [
    Registry {
        name: "hub.docker.com",
        caption: "官方镜像",
        url: "https://registry.hub.docker.com",
    },
    Registry {
        name: "163",
        caption: "网易镜像",
        url: "http://hub-mirror.c.163.com",
    },
    Registry {
        name: "ustc",
        caption: "中国科学技术大学",
        url: "https://docker.mirrors.ustc.edu.cn",
    },
    Registry {
        name: "tencentyun",
        caption: "腾讯云",
        url: "https://mirror.ccs.tencentyun.com",
    },	
	Registry {
        name: "docker-cn",
        caption: "docker中国",
        url: "https://registry.docker-cn.com",
    },
];

fn action_setup_mirror(
    _parent: Option<&dyn Module>,
    param: &ArgMatches,
) -> Result<(), anyhow::Error> {
	if param.get_flag("list") {
        list_registers(&REGISTRYS);
        return Ok(());
    }

	registry::setup_proxy_action(param, "mirror", &REGISTRYS, |registry|{
		let mut lines = Vec::<String>::new();
		lines.push("{\n".to_string());
		lines.push("\t\"registry-mirrors\": [ \n".to_string());
		lines.push(format!("\t\t\"{}\"\n" , registry.url));
		lines.push("\t]\n".to_string());
		lines.push("}\n".to_string());

		write_lines_to_file(
			&lines,
			Path::new("/etc/docker/"),
			"daemon.json",
			"daemon.json.bak",
		)?;
		let command = Cmd {
			cmd: "sudo",
			params: vec!["systemctl", "restart", "docker"],
		};
	
		if let Ok(code) = execute_command(&command) {
			if 0 == code {
				println!("exec {} succeed", command.to_string());
			}
		}
		Ok(())
	})
}


fn action_setup_user(
    _parent: Option<&dyn Module>,
    param: &ArgMatches,
) -> Result<(), anyhow::Error> {
    if let Some(user) = param.value_of("user") {
		let cmd_add_user_to_group = Cmd {
			cmd: "sudo",
			params: vec![ "usermod", "-aG", "docker", user],
		};
	
		if let Ok(code) = execute_command(&cmd_add_user_to_group) {
			if 0 == code {
				println!("exec {} succeed", cmd_add_user_to_group.to_string());

				 
				let cmd_refresh_group = Cmd {
					cmd: "newgrp",
					params: vec!["docker"],
				};
				if let Ok(code) = execute_command(&cmd_refresh_group) {
					if 0 == code {
						println!("exec {} succeed", cmd_refresh_group.to_string());
					}
				}

				let cmd_restart_docker = Cmd {
					cmd: "sudo",
					params: vec!["systemctl", "restart", "docker"],
				};
			
				if let Ok(code) = execute_command(&cmd_restart_docker) {
					if 0 == code {
						println!("exec {} succeed", cmd_restart_docker.to_string());
					}
				}				
				return Ok(());
			} else {
				return Err(anyhow!("failed to execute command"));
			}
		}
    } else {
        return Err(anyhow!("miss param for user"));
    }	
	Ok(())
}

