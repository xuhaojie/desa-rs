use crate::{BaseModule, BasicAction, Module};
use clap::{Arg, ArgMatches, Command};
use dirs;
use reqwest::Url;
use std::io::{self, prelude::*, BufWriter};
use utility::{clean::*, platform::*};

pub fn new() -> Box<dyn Module> {
    Box::new(BaseModule {
        name: "pip",
        description: "Setup pip",
        actions: vec![BasicAction {
            name: "proxy",
            cmd: || {
                Command::new("download")
                    .about("controls testing features")
                    .arg(
                        Arg::new("mirror")
                            .short('m')
                            .long("mirror")
                            .help("set mirror name, [tuna, 163, aliyun]")
                            .takes_value(true),
                    )
            },
            execute: action_setup_proxy,
        }],
    })
}

fn action_setup_proxy(parent: Option<&dyn Module>, param: &ArgMatches) -> std::io::Result<()> {
    let mut lines = vec![
        "[global]\n",
        "index-url=https://pypi.tuna.tsinghua.edu.cn/simple\n",
        "[install]\n",
        "trusted-host=https://pypi.tuna.tsinghua.edu.cn\n",
    ];

    //# 如：
    let mirros = ["tuna", "163", "aliyun"];

    if let Some(mirror) = param.value_of("mirror") {
        let mut find = false;
        for m in mirros.iter() {
            if *m == mirror {
                find = true;
                break;
            }
        }
        if find {
            let url = match mirror {
                "tuna" => "https://pypi.tuna.tsinghua.edu.cn/simple",
                "163" => "https://mirrors.163.com/pypi/simple",
                "aliyun" => "http://mirrors.aliyun.com/pypi/simple",
                _ => "https://pypi.tuna.tsinghua.edu.cn/simple",
            };
            let url = Url::parse(&url).unwrap();

            let set = format!("index-url={}\n", url.to_string());
            lines[1] = &set;

            let set = format!("trusted-host={}\n", url.host().unwrap().to_string());
            lines[3] = &set;

            let home_dir = match dirs::home_dir() {
                Some(path) => path,
                None => return Err(io::Error::new(io::ErrorKind::Other, "can't get home dir")),
            };

            let (folder_name, file_name, backup_file) = match current_platform() {
                Platform::LINUX | Platform::MACOS => (".pip", "pip.conf", "pip.conf.bak"),
                Platform::WINDOWS => ("pip", "pip.ini", "pip.ini.bak"),
                _ => return Err(io::Error::new(io::ErrorKind::Other, "unsupported platform")),
            };

            let target_path = home_dir.join(folder_name);
            let target_file = target_path.join(file_name);
            let backup_file = target_path.join(backup_file);
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
