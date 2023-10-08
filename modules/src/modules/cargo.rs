use crate::{BasicAction, BasicActionManager, Module};
use clap::{Arg, ArgMatches, Command, SubCommand};
use dirs;
use std::io::{self, prelude::*, BufWriter};
use utility::clean::*;

struct CargoModule {
    actions: Vec<BasicAction<CargoModule>>,
}

fn register_actions<T>(sub_command: &str, actions: &Vec<BasicAction<T>>) -> Command<'static> {
    let mut cmd = Command::new(sub_command).about("setup cargo");
    for action in actions {
        cmd = cmd.subcommand((action.cmd)());
    }
    cmd
}

fn execute_action<T>(
    module: &T,
    actions: &Vec<BasicAction<T>>,
    param: &ArgMatches,
) -> std::io::Result<()> {
    if let Some(action) = param.subcommand() {
        for act in actions {
            if act.name == action.0 {
                if let Some(param) = param.subcommand_matches(act.name) {
                    return (act.execute)(module, param);
                }
            }
        }
    };
    return Err(io::Error::new(
        io::ErrorKind::Other,
        format!("require sub command for '{}'", "cargo"),
    ));
}
impl Module for CargoModule {
    fn name(&self) -> &'static str {
        "cargo"
    }

    fn command(&self) -> Command<'static> {
        register_actions("cargo", &self.actions)
        /*
        let cmd = Command::new(self.name()).about("setup cargo");
        let mut c: Command = cmd;
        for action in &self.actions {
            c = c.subcommand((action.cmd)());
        }
        c
        */
    }

    fn execute(&self, param: &ArgMatches) -> std::io::Result<()> {
        execute_action(self, &self.actions, param)
        /*/
        if let Some(action) = param.subcommand() {
            //println!("{}", action.0);
            //return self.action_manager.execute_action(action, self, param);

            for act in &self.actions {
                if act.name == action.0 {
                    if let Some(param) = param.subcommand_matches(act.name) {
                        return (act.execute)(self, param);
                    }
                }
            }
        };
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("require sub command for '{}'", self.name()),
        ));
        */
    }
}

pub fn new() -> Box<dyn Module> {
    Box::new(CargoModule {
        //    action_manager: BasicActionManager {
        actions: vec![
            BasicAction {
                name: "clean",
                cmd: || {
                    Command::new("clean").arg(
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
                    Command::new("proxy").arg(
                        Arg::new("mirror")
                            .short('m')
                            .long("mirror")
                            .help("mirror name, one of tuna, sjtu, ustc, rustcc")
                            .takes_value(true),
                    )
                },
                execute: action_setup_proxy,
            },
        ],
        //},
    })
}

fn action_clean(module: &CargoModule, param: &ArgMatches) -> std::io::Result<()> {
    let path = match param.value_of("path") {
        Some(p) => p.to_owned(),
        //None => return Err(io::Error::new(io::ErrorKind::Other,"please specify a path")),
        None => String::from(std::env::current_dir()?.as_path().to_str().unwrap()),
    };

    let mut projects = Vec::<String>::new();

    search_projects(&path, "Cargo.toml", &mut projects)?;
    return clean_projects(&projects, "cargo", &["clean"]);
}

fn action_setup_proxy(module: &CargoModule, param: &ArgMatches) -> std::io::Result<()> {
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

    //# 如：
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
                std::fs::create_dir(target_path);
            }
            if target_file.exists() {
                if backup_file.exists() {
                    std::fs::remove_file(backup_file.as_path());
                }
                std::fs::rename(target_file.as_path(), backup_file.as_path());
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
