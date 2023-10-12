use crate::{BaseModule, BasicAction, Module};
use anyhow::anyhow;
use clap::{Arg, ArgMatches, Command};
use utility::execute::{execute_command, Cmd};
use utility::registry::{self,Registry, list_registers, set_registry};

pub fn new() -> Box<dyn Module> {
    Box::new(BaseModule {
        name: "npm",
        description: "Setup npm proxy",
        actions: vec![BasicAction {
            name: "proxy",
            cmd: || {
                Command::new("proxy")
                    .about("clean cargo projects builds")
                    .arg(
                        Arg::new("mirror")
                            .help("mirror name, [taobao, origin]")
                            .takes_value(true),
                    )
            },
            execute: action_setup_proxy,
        }],
    })
}

static REGISTRYS:[Registry;2] = [
    Registry {
        name: "nmpjs",
        caption: "官方镜像",
        url: "https://registry.npmjs.org/",
    },
    Registry {
        name: "taobao",
        caption: "淘宝镜像",
        url: "https://registry.npm.taobao.org",
    },
];

fn action_setup_proxy(
    _parent: Option<&dyn Module>,
    param: &ArgMatches,
) -> Result<(), anyhow::Error> {
	registry::setup_proxy_action(param,&REGISTRYS,|registry|{
		let cmd = Cmd {
			cmd: "npm",
			params: vec!["config", "set", "registry", registry.url],
		};
		if let Ok(code) = execute_command(&cmd) {
			if 0 == code {}
		}
		Ok(())
	})
}
