use crate::{BaseModule, BasicAction, Module};
use anyhow::anyhow;
use clap::{Arg, ArgMatches, Command};
use utility::mirror::{self, Mirror};
use utility::{clean::*, execute::*};

pub fn new() -> Box<dyn Module> {
    Box::new(BaseModule {
        name: "go",
        description: "Setup go proxy or clean go projects",
        actions: vec![
            BasicAction {
                name: "clean",
                cmd: || {
                    Command::new("clean")
                        .about("clean go project builds recursively")
                        .arg(
                            Arg::new("path")
                                .short('p')
                                .long("path")
                                .help("Set start path")
                                .takes_value(true),
                        )
                },
                execute: action_clean,
            },
            BasicAction {
                name: "mirror",
                cmd: || {
                    Command::new("mirror")
                        .about("clean cargo projects builds")
                        .arg(
                            Arg::new("mirror")
                                .help("mirror name, [goproxy.cn, goproxy.io]")
                                .takes_value(true),
                        )
                        .arg(
                            Arg::new("list")
                                .short('l')
                                .long("list")
                                .help("list available go proxys")
                                .action(clap::ArgAction::SetTrue),
                        )
                },
                execute: action_setup_proxy,
            },
        ],
    })
}

fn action_clean(_parent: Option<&dyn Module>, param: &ArgMatches) -> Result<(), anyhow::Error> {
    let path = match param.value_of("path") {
        Some(p) => p.to_owned(),
        None => String::from(std::env::current_dir()?.as_path().to_str().unwrap()),
        //None => return Err(io::Error::new(io::ErrorKind::Other,"please specify a path")),
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

static MIRRORS: [Mirror; 2] = [
    Mirror {
        name: "goproxy.cn",
        caption: "goproxy.cn",
        url: "https://goproxy.cn,direct",
    },
    Mirror {
        name: "goproxy.io",
        caption: "goproxy.io",
        url: "https://proxy.golang.com.cn,direct",
    },
];

fn action_setup_proxy(
    _parent: Option<&dyn Module>,
    param: &ArgMatches,
) -> Result<(), anyhow::Error> {
    mirror::setup_mirror_action(param, "mirror", &MIRRORS, |mirror| {
        let cmd1 = Cmd {
            cmd: "go",
            params: vec!["env", "-w", "GO111MODULE=on"],
        };
        if let Ok(code) = execute_command(&cmd1) {
            if 0 == code {
                println!("exec {} succeed", cmd1.to_string());
            }
        } else {
            return Err(anyhow!(
                "exec \"{}\" failed! Please install go first!",
                cmd1.to_string(),
            ));
        }

        let proxy = format!("GOPROXY={}", mirror.url);
        let cmd2 = Cmd {
            cmd: "go",
            params: vec!["env", "-w", &proxy],
        };
        if let Ok(code) = execute_command(&cmd2) {
            if 0 == code {
                println!("exec {} succeed", cmd2.to_string());
            }
        } else {
            return Err(anyhow!(
                "exec \"{}\" failed! Please install go first!",
                cmd1.to_string(),
            ));
        }
        Ok(())
    })
}
