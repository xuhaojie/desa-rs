use crate::{BaseModule, BasicAction, Module};
use clap::{Arg, ArgMatches, Command};
use dirs;
use std::io::{self, prelude::*, BufWriter};
use utility::clean::*;

pub fn new() -> Box<dyn Module> {
    Box::new(BaseModule {
        name: "cargo",
        description: "Setup mirror or clean cargo projects",
        actions: vec![
            BasicAction {
                name: "clean",
                cmd: || {
                    Command::new("clean")
                        .about("clean cargo project builds recursively")
                        .arg(
                            Arg::new("path")
                                .short('p')
                                .long("path")
                                .help("set start path for clean")
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
                                .help("mirror name, [tuna, sjtu, ustc, rustcc]")
                                .takes_value(true),
                        )
                },
                execute: action_setup_proxy,
            },
        ],
    })
}

fn action_clean(_parent: Option<&dyn Module>, param: &ArgMatches) -> std::io::Result<()> {
    let path = match param.value_of("path") {
        Some(p) => p.to_owned(),
        //None => return Err(io::Error::new(io::ErrorKind::Other,"please specify a path")),
        None => String::from(std::env::current_dir()?.as_path().to_str().unwrap()),
    };

    let mut projects = Vec::<String>::new();

    search_projects(&path, "Cargo.toml", &mut projects)?;
    return clean_projects(&projects, "cargo", &["clean"]);
}

fn action_setup_proxy(_parent: Option<&dyn Module>, param: &ArgMatches) -> std::io::Result<()> {
    let mut lines = vec![
        "[source.crates-io]\n",
        "registry =\"https://github.com/rust-lang/crates.io-index\"\n",
        "# 指定镜像\n",
        "replace-with = '镜像源名'\n",
        "# 中国科学技术大学\n",
        "[source.ustc]\n",
        "registry = \"https://mirrors.ustc.edu.cn/crates.io-index\"\n\n",
        "# 上海交通大学\n",
        "[source.sjtu]\n",
        "registry = \"https://mirrors.sjtug.sjtu.edu.cn/git/crates.io-index\"\n\n",
        "# 清华大学\n",
        "[source.tuna]\n",
        "registry = \"https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git\"\n\n",
        "# rustcc社区\n",
        "[source.rustcc]\n",
        "registry = \"https://code.aliyun.com/rustcc/crates.io-index.git\"\n\n",
    ];

    let mirros = ["tuna", "sjtu", "ustc", "rustcc"];

    if let Some(mirror) = param.value_of("mirror") {
        let mut find = false;
        for m in mirros.iter() {
            if *m == mirror {
                find = true;
                break;
            }
        }
        if find {
            let set = format!("replace-with = \"{}\"\n", mirror).to_string();
            lines[3] = &set;

            let home_dir = match dirs::home_dir() {
                Some(path) => path,
                None => return Err(io::Error::new(io::ErrorKind::Other, "can't get home dir")),
            };

            let target_path = home_dir.join(".cargo");
            let target_file = target_path.join("config");
            let backup_file = target_path.join("config.bak");
            if !target_path.exists() {
                let _ = std::fs::create_dir(target_path);
            }
            if target_file.exists() {
                if backup_file.exists() {
                    let _ = std::fs::remove_file(backup_file.as_path());
                }
                let _ = std::fs::rename(target_file.as_path(), backup_file.as_path());
            }

            let mut buffer = BufWriter::new(std::fs::File::create(target_file)?);
            for line in lines.iter() {
                buffer.write_all(line.as_bytes())?;
            }
            buffer.flush()?;
            println!("set proxy to {} succeeded", mirror);
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "invalid mirror"))
        }
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "miss param for mirror",
        ))
    }
}
