use crate::{BaseModule, BasicAction, Module};
use anyhow::anyhow;
use clap::{Arg, ArgMatches, Command};
use dirs;
use reqwest::Url;
use std::io::{prelude::*, BufWriter};
use utility::execute::{execute_command, Cmd};
use utility::platform::*;
use utility::registry::{self,Registry, list_registers, set_registry};

pub fn new() -> Box<dyn Module> {
    Box::new(BaseModule {
        name: "pip",
        description: "Setup pip",
        actions: vec![BasicAction {
            name: "mirror",
            cmd: || {
                Command::new("mirror")
                    .about("controls testing features")
                    .arg(
                        Arg::new("mirror")
                            //.short('m')
                            //.long("mirror")
                            .help("set mirror name, [tuna, 163, aliyun]")
                            .takes_value(true),
                    )
                    .arg(
                        Arg::new("list")
                            .short('l')
                            .long("list")
                            .help("list available pip mirrors")
                            .action(clap::ArgAction::SetTrue),
                    )					
            },
            execute: action_setup_proxy,
        }],
    })
}

static REGISTRYS:[Registry;4] = [
    Registry {
        name: "pypi.org",
        caption: "官方镜像",
        url: "https://pypi.org/simple",
    },
    Registry {
        name: "tuna",
        caption: "清华·镜像",
        url: "https://pypi.tuna.tsinghua.edu.cn/simple",
    },
    Registry {
        name: "163",
        caption: "网易镜像",
        url: "https://mirrors.163.com/pypi/simple",
    },
    Registry {
        name: "aliyun",
        caption: "淘宝镜像",
        url: "http://mirrors.aliyun.com/pypi/simple",
    },		
];

fn action_setup_proxy(
    _parent: Option<&dyn Module>,
    param: &ArgMatches,
) -> Result<(), anyhow::Error> {
	registry::setup_proxy_action(param,"mirror",&REGISTRYS,|registry|{
		let cmd = Cmd {
			cmd: "pip",
			params: vec!["config", "set", "global.index-url", registry.url],
		};
		if let Ok(code) = execute_command(&cmd) {
			if 0 == code {}
		}
		Ok(())
	})
}
