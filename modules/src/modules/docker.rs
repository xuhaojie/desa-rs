use super::{BaseModule, BasicAction, Module};
use anyhow::anyhow;
use clap::{Arg, ArgMatches, Command};
use std::path::Path;
use utility::{
    execute::{execute_command, Cmd},
    file::write_lines_to_file,
	mirror::{self, Mirror, list_mirrors},
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
					.help("mirror name, use -l to list")
					.takes_value(true),
			)
			.arg(
				Arg::new("list")
					.short('l')
					.long("list")
					.help("list available cargo mirrors")
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

static MIRRORS:[Mirror;5] = [
    Mirror {
        name: "hub.docker.com",
        caption: "官方镜像",
        url: "https://mirror.hub.docker.com",
    },
    Mirror {
        name: "163",
        caption: "网易镜像",
        url: "http://hub-mirror.c.163.com",
    },
    Mirror {
        name: "ustc",
        caption: "中国科学技术大学",
        url: "https://docker.mirrors.ustc.edu.cn",
    },
    Mirror {
        name: "tencentyun",
        caption: "腾讯云",
        url: "https://mirror.ccs.tencentyun.com",
    },	
	Mirror {
        name: "docker-cn",
        caption: "docker中国",
        url: "https://mirror.docker-cn.com",
    },
];

fn action_setup_mirror(
    param: &ArgMatches,
) -> Result<(), anyhow::Error> {
	if param.get_flag("list") {
        list_mirrors(&MIRRORS);
        return Ok(());
    }

	mirror::setup_mirror_action(param, "mirror", &MIRRORS, |mirror|{
		let mut lines = Vec::<String>::new();
		lines.push("{\n".to_string());
		lines.push(format!("  \"registry-mirrors\": [\"{}\"]\n" , mirror.url));
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

