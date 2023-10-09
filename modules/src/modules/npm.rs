use crate::{BaseModule, BasicAction, Module};
use clap::{Arg, ArgMatches, Command};
use std::io;
use utility::execute::*;

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
                            .short('m')
                            .long("mirror")
                            .help("mirror name, [taobao, origin]")
                            .takes_value(true),
                    )
            },
            execute: action_setup_proxy,
        }],
    })
}

fn action_setup_proxy(_parent: Option<&dyn Module>, param: &ArgMatches) -> std::io::Result<()> {
    let mirros = ["origin", "taobao"];
    if let Some(mirror) = param.value_of("mirror") {
        let mut target = -1;
        let mut index = 0;
        for m in mirros.iter() {
            if *m == mirror {
                target = index;
                break;
            }
            index += 1;
        }

        if target < 0 {
            return Err(io::Error::new(io::ErrorKind::Other, "invalid mirror"));
        } else {
            let url = match mirror {
                "origin" => "https://registry.npmjs.org/",
                "taobao" => "https://registry.npm.taobao.org",
                _ => "https://registry.npmjs.org/",
            };

            let cmd = Cmd {
                cmd: "npm",
                params: vec!["config", "set", "registry", url],
            };
            if let Ok(code) = execute_command(&cmd) {
                if 0 == code {}
            }
            println!("set proxy to {} succeeded", mirror);
            Ok(())
        }
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "miss param for mirror",
        ))
    }
}
