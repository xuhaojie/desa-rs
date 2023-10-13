use crate::{BaseModule, BasicAction, Module};
use anyhow::anyhow;
use clap::{Arg, ArgMatches, Command};
use utility::execute::{execute_command, Cmd};
use utility::mirror::{self, Mirror};

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

static MIRRORS: [Mirror; 4] = [
    Mirror {
        name: "pypi.org",
        caption: "官方镜像",
        url: "https://pypi.org/simple",
    },
    Mirror {
        name: "tuna",
        caption: "清华·镜像",
        url: "https://pypi.tuna.tsinghua.edu.cn/simple",
    },
    Mirror {
        name: "163",
        caption: "网易镜像",
        url: "https://mirrors.163.com/pypi/simple",
    },
    Mirror {
        name: "aliyun",
        caption: "淘宝镜像",
        url: "http://mirrors.aliyun.com/pypi/simple",
    },
];

fn action_setup_proxy(
    _parent: Option<&dyn Module>,
    param: &ArgMatches,
) -> Result<(), anyhow::Error> {
    mirror::setup_mirror_action(param, "mirror", &MIRRORS, |mirror| {
        let cmd = Cmd {
            cmd: "pip",
            params: vec!["config", "set", "global.index-url", mirror.url],
        };
        if let Ok(code) = execute_command(&cmd) {
            if 0 == code {}
        } else {
            return Err(anyhow!(
                "exec \"{}\" failed! Please install pip first!",
                cmd.to_string(),
            ));
        }
        Ok(())
    })
}
