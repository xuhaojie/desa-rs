use crate::{BaseModule, BasicAction, Module};
use clap::{Arg, ArgMatches, Command};
use dirs;
use std::io::{self, prelude::*, BufWriter};
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
                name: "proxy",
                cmd: || {
                    Command::new("proxy")
                        .about("clean cargo projects builds")
                        .arg(
                            Arg::new("mirror")
                                .short('m')
                                .long("mirror")
                                .help("mirror name, [goproxy.cn, goproxy.io]")
                                .takes_value(true),
                        )

                },
                execute: action_setup_proxy,
            },
        ],
    })
}


fn action_clean(parent: Option<&dyn Module>, param: &ArgMatches) -> std::io::Result<()> {
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

fn action_setup_proxy(parent: Option<&dyn Module>, param: &ArgMatches) -> std::io::Result<()> {
    //$ go env -w GO111MODULE=on
    //$ go env -w GOPROXY=https://goproxy.cn,direct
    let mirros = ["goproxy.cn", "goproxy.io"];
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
                "goproxy.cn" => "https://goproxy.cn,direct",
                "goproxy.io" => "https://proxy.golang.com.cn,direct",
                _ => "https://goproxy.cn,direct",
            };

            let cmd1 = Cmd {
                cmd: "go",
                params: vec!["env", "-w", "GO111MODULE=on"],
            };
            //let cmd = format!("git config --global user.name {}", user);

            if let Ok(code) = execute_command(&cmd1) {
                if 0 == code {
                    println!("exec {} succeed", cmd1.to_string());
                }
            }
            let proxy = format!("GOPROXY={}", url);
            let cmd2 = Cmd {
                cmd: "go",
                params: vec!["env", "-w", &proxy],
            };
            if let Ok(code) = execute_command(&cmd2) {
                if 0 == code {
                    println!("exec {} succeed", cmd2.to_string());
                }
            }
            Ok(())
        }
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "miss param for mirror",
        ))
    }
}
