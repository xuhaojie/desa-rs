use crate::{BaseModule, BasicAction, Module};
use anyhow::anyhow;
use clap::{Arg, ArgMatches, Command};
use dirs;
use utility::{clean::*, file::write_lines_to_file};
use utility::mirror::{self,Mirror, list_mirrors};

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
                name: "mirror",
                cmd: || {
                    Command::new("mirror")
                        .about("set cargo mirror")
                        .arg(
                            Arg::new("mirror")
                                //.short('m')
                                //.long("mirror")
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

static MIRRORS: [Mirror; 5] = [
    Mirror {
        name: "crates-io",
        caption: "官方镜像",
        url: "https://github.com/rust-lang/crates.io-index",
    },
    Mirror {
        name: "ustc",
        caption: "中国科学技术大学",
        url: "https://mirrors.ustc.edu.cn/crates.io-index",
    },
    Mirror {
        name: "sjtu",
        caption: "上海交通大学",
        url: "https://mirrors.sjtug.sjtu.edu.cn/git/crates.io-index",
    },
    Mirror {
        name: "tuna",
        caption: "清华大学",
        url: "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git",
    },
    Mirror {
        name: "rustcc",
        caption: "rustcc社区",
        url: "https://code.aliyun.com/rustcc/crates.io-index.git",
    },
];

fn gen_config(mirror: &Mirror) -> Vec<String> {
    let mut result = Vec::<String>::new();
    let mut i = 0;

    if mirror.name == MIRRORS[0].name {
        let r = &MIRRORS[0];
        result.push(format!(
            "# {}\n[source.{}]\nMirror = \"{}\"\n",
            r.caption, r.name, r.url
        ));
    } else {
        for r in &MIRRORS {
            result.push(format!(
                "# {}\n[source.{}]\nMirror = \"{}\"\n",
                r.caption, r.name, r.url
            ));
            if i == 0 {
                result.push(format!("replace-with = \"{}\"\n", mirror.name));
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
        list_mirrors(&MIRRORS);
        return Ok(());
    }
	mirror::setup_mirror_action(param, "mirror", &MIRRORS,|mirror|{
		let lines = gen_config(mirror);
		for l in &lines {
			print!("{}", l);
		}

		let home_dir = match dirs::home_dir() {
			Some(path) => path,
			None => return Err(anyhow!("can't get home dir")),
		};

		let target_path = home_dir.join(".cargo");
		write_lines_to_file(&lines, &target_path, "config", "config.bak")?;
		println!("set proxy to {} succeeded", mirror.name);
		Ok(())
	})

}
