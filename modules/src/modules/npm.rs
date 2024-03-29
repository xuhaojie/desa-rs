use super::{BaseModule, BasicAction, Module};
use anyhow::anyhow;
use clap::{Arg, ArgMatches, Command};
use utility::execute::{execute_command, Cmd};
use utility::mirror::{self, Mirror};

pub fn new() -> Box<dyn Module> {
    Box::new(BaseModule {
        name: "npm",
        description: "Setup npm proxy",
        actions: vec![BasicAction {
            name: "mirror",
            cmd: || {
                Command::new("mirror")
                    .about("setup npm mirror")
                    .arg(
                        Arg::new("mirror")
                            .help("mirror name, for example npmmirror")
                            .takes_value(true),
                    )
                    .arg(
                        Arg::new("list")
                            .short('l')
                            .long("list")
                            .help("list available npm mirrors")
                            .action(clap::ArgAction::SetTrue),
                    )
            },
            execute: action_setup_proxy,
        }],
    })
}

static MIRRORS: [Mirror; 2] = [
    Mirror {
        name: "nmpjs",
        caption: "官方镜像",
        url: "https://registry.npmjs.org/",
    },
    Mirror {
        name: "npmmirror",
        caption: "淘宝镜像",
        url: "https://registry.npmmirror.com",
    },
];

fn action_setup_proxy(
    param: &ArgMatches,
) -> Result<(), anyhow::Error> {
    mirror::setup_mirror_action(param, "mirror", &MIRRORS, |mirror| {
        let cmd = Cmd {
            cmd: "npm",
            params: vec!["config", "set", "registry", mirror.url],
        };
        if let Ok(code) = execute_command(&cmd) {
            if 0 == code {}
        } else {
            return Err(anyhow!(
                "exec \"{}\" failed! Please install npm first!",
                cmd.to_string(),
            ));
        }
        Ok(())
    })
}
