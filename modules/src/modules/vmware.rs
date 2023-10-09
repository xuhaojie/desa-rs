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

fn action_download(_parent: Option<&dyn Module>, param: &ArgMatches) -> std::io::Result<()> {
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
    println!("target url: {}", target_url);
    let target_folder = std::path::Path::new("./");
    download_file_to_folder(target_url.as_str(), target_folder, true)
}
