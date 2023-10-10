use crate::{BaseModule, BasicAction, Module};
use anyhow::anyhow;
use clap::{Arg, ArgMatches, Command};
use dirs;
use utility::{clean::*, file::write_lines_to_file};

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
                        .arg(
                            Arg::new("list")
                                .short('l')
                                .long("list")
                                .help("list available cargo registers")
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
        //None => return Err(io::Error::new(io::ErrorKind::Other,"please specify a path")),
        None => String::from(std::env::current_dir()?.as_path().to_str().unwrap()),
    };

    let mut projects = Vec::<String>::new();

    search_projects(&path, "Cargo.toml", &mut projects)?;
    return clean_projects(&projects, "cargo", &["clean"]);
}

struct Registry {
    name: &'static str,
    caption: &'static str,
    url: &'static str,
}

static REGISTRYS: [Registry; 5] = [
    Registry {
        name: "crates-io",
        caption: "官方镜像",
        url: "https://github.com/rust-lang/crates.io-index",
    },
    Registry {
        name: "ustc",
        caption: "中国科学技术大学",
        url: "https://mirrors.ustc.edu.cn/crates.io-index",
    },
    Registry {
        name: "sjtu",
        caption: "上海交通大学",
        url: "https://mirrors.sjtug.sjtu.edu.cn/git/crates.io-index",
    },
    Registry {
        name: "tuna",
        caption: "清华大学",
        url: "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git",
    },
    Registry {
        name: "rustcc",
        caption: "rustcc社区",
        url: "https://code.aliyun.com/rustcc/crates.io-index.git",
    },
];

fn list_registers() {
    for r in &REGISTRYS {
        print!("{} [{}] \n{}\n", r.caption, r.name, r.url);
    }
}

fn gen_config(index: usize) -> Vec<String> {
    let mut result = Vec::<String>::new();
    let mut i = 0;

    if index == 0 {
        let r = &REGISTRYS[0];
        result.push(format!(
            "# {}\n[source.{}]\nregistry = \"{}\"\n",
            r.caption, r.name, r.url
        ));
    } else {
        for r in &REGISTRYS {
            result.push(format!(
                "# {}\n[source.{}]\nregistry = \"{}\"\n",
                r.caption, r.name, r.url
            ));
            if i == 0 {
                result.push(format!("replace-with = \"{}\"\n", REGISTRYS[index].name));
            }
            i += 1;
        }
    }
    result
}

fn action_setup_proxy(
    _parent: Option<&dyn Module>,
    param: &ArgMatches,
) -> Result<(), anyhow::Error> {
    if param.get_flag("list") {
        list_registers();
        return Ok(());
    }

    if let Some(mirror) = param.value_of("mirror") {
        let mut index: i32 = -1;
        let mut i = 0;
        for r in REGISTRYS.iter() {
            if r.name == mirror {
                index = i;
                break;
            }
            i += 1;
        }

        if index >= 0 {
            let lines = gen_config(index as usize);
            for l in &lines {
                print!("{}", l);
            }

            let home_dir = match dirs::home_dir() {
                Some(path) => path,
                None => return Err(anyhow!("can't get home dir")),
            };

            let target_path = home_dir.join(".cargo");
            write_lines_to_file(&lines, &target_path, "config", "config.bak")?;
            println!("set proxy to {} succeeded", mirror);
            Ok(())
        } else {
            Err(anyhow!("invalid mirror"))
        }
    } else {
        list_registers();
        Err(anyhow!(
            "Please specify a registery by name, for example tuna"
        ))
    }
}
