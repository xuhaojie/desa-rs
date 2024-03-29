use super::{BaseModule, BasicAction, Module}; // or use crate::modules::{BaseModule, BasicAction, Module};
use clap::{Arg, ArgMatches, Command};
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

use utility::file::*;
use utility::mirror::{self, list_mirrors, Mirror};
pub fn new() -> Box<dyn Module> {
    Box::new(BaseModule {
        name: "apt",
        description: "Setup apt mirror",
        actions: vec![BasicAction {
            name: "mirror",
            cmd: || {
                Command::new("mirror")
                    .about("setup apt mirror")
                    .arg(
                        Arg::new("mirror")
                            .help("mirror name, [cn.ubuntu, tuna]")
                            .takes_value(true),
                    )
                    .arg(
                        Arg::new("list")
                            .short('l')
                            .long("list")
                            .help("list available apt mirrors")
                            .action(clap::ArgAction::SetTrue),
                    )
            },
            execute: action_setup_proxy,
        }],
    })
}

static MIRRORS: [Mirror; 2] = [
    Mirror {
        name: "cn.ubuntu",
        caption: "官方中国镜像",
        url: "http://cn.archive.ubuntu.com/ubuntu/",
    },
    Mirror {
        name: "tuna",
        caption: "清华大学",
        url: "https://mirrors.tuna.tsinghua.edu.cn/ubuntu/",
    },
];

fn get_codename() -> Option<&'static str> {
    let Ok(input) = File::open("/etc/issue") else {
        return None;
    };
    let mut buffered = BufReader::new(input);
    let mut line = String::new();
    let Ok(_) = buffered.read_line(&mut line) else {
        return None;
    };

    let tokens: Vec<&str> = line.split_whitespace().collect();

    if tokens.len() < 3 || tokens[0] != "Ubuntu" {
        return None;
    }
    let version = &tokens[1][0..5];

    match version {
        "22.04" => Some("jammy"),
        "23.04" => Some("lunar"),
        "22.10" => Some("kinetic"),
        "18.04" => Some("bionic"),
        "16.04" => Some("xenial"),
        "14.04" => Some("trusty"),
        "20.04" => Some("focal"),
        _ => None,
    }
}

#[rustfmt::skip] 
fn gen_ubuntu_apt_config(mirror: &Mirror, codename: &str) -> Vec<String> {
    let mut result = Vec::<String>::new();

    result.push("# 默认注释了源码镜像以提高 apt update 速度，如有需要可自行取消注释\n".to_string());

    result.push(format!("deb {} {} main restricted universe multiverse\n", mirror.url, codename));
    result.push(format!("# deb-src {} {} main restricted universe multiverse\n", mirror.url, codename));
	result.push(format!("deb {} {}-updates main restricted universe multiverse\n", mirror.url, codename));
    result.push(format!("# deb-src {} {}-updates main restricted universe multiverse\n", mirror.url, codename));
    result.push(format!("deb {} {}-backports main restricted universe multiverse\n", mirror.url, codename));
    result.push(format!("# deb-src {} {}-updates main restricted universe multiverse\n", mirror.url, codename));

    result.push("\n".to_string());

    result.push(format!("deb {} {}-security main restricted universe multiverse\n",mirror.url, codename));
    result.push(format!("# deb-src {} {}-security main restricted universe multiverse\n", mirror.url, codename));

    result.push("\n".to_string());

    result.push(format!("deb http://security.ubuntu.com/ubuntu/ {}-security main restricted universe multiverse\n", codename ));
    result.push(format!("# deb-src http://security.ubuntu.com/ubuntu/ {}-security main restricted universe multiverse\n",codename));

    result.push("\n".to_string());

    result.push("# 预发布软件源，不建议启用\n".to_string());

    result.push(format!("# deb {} {}-proposed main restricted universe multiverse\n", mirror.url, codename));
    result.push(format!("# # deb-src {} {}-proposed main restricted universe multiverse\n", mirror.url, codename));

    result
}

fn action_setup_proxy(
    param: &ArgMatches,
) -> Result<(), anyhow::Error> {
    if param.get_flag("list") {
        list_mirrors(&MIRRORS);
        return Ok(());
    }
    mirror::setup_mirror_action(param, "mirror", &MIRRORS, |mirror| {
        let Some(code_name) = get_codename() else {
            return Ok(());
        };

        println!("code_name: {}", code_name);
        let lines = gen_ubuntu_apt_config(mirror, code_name);
        write_lines_to_file(
            &lines,
            Path::new("/etc/apt/"),
            "sources.list",
            "sources.list.bak",
        )?;
        Ok(())
    })
}
