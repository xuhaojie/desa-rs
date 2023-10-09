use crate::{BaseModule, BasicAction, Module};
use clap::{Arg, ArgMatches, Command};
use std::io;
use utility::{download::*, platform::*};

pub fn new() -> Box<dyn Module> {
    Box::new(BaseModule {
        name: "vmware",
        description: "Download vmware",
        actions: vec![BasicAction {
            name: "download",
            cmd: || {
                Command::new("download").about("download vmware").arg(
                    Arg::new("os")
                        .short('o')
                        .long("os")
                        .help("os type,[linux, macos ,windows]")
                        .takes_value(true),
                )
            },
            execute: action_download,
        }],
    })
}

fn action_download(parent: Option<&dyn Module>, param: &ArgMatches) -> std::io::Result<()> {
    if let Some(parent) = parent {
        println!("download action in {}", parent.name());
    }
    let os = match param.value_of("os") {
        Some(os) => Platform::from(os),
        None => current_platform(),
    };

    let url = match os {
        Platform::WINDOWS => "https://www.vmware.com/go/getworkstation-win",
        Platform::LINUX => "https://www.vmware.com/go/getworkstation-linux",
        Platform::MACOS => "https://www.vmware.com/go/getfusion",
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("not support os type {}", os),
            ));
        }
    };
    let target_url = utility::download::get_redirected_url(url)?;
    println!("get target url: {}", target_url);
    let target_folder = std::path::Path::new("./");
    download_file(target_url.as_str(), target_folder, true)
}
