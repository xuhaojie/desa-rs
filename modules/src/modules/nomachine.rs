use crate::{BaseModule, BasicAction, Module};
use anyhow::anyhow;
use clap::{Arg, ArgMatches, Command};
use utility::{arch::*, download::*, package::*, platform::*};
pub fn new() -> Box<dyn Module> {
    Box::new(BaseModule {
        name: "nomachine",
        description: "Download nomachine",
        actions: vec![BasicAction {
            name: "download",
            cmd: || {
                Command::new("download")
                    .about("download nomachine")
                    .arg(
                        Arg::new("os")
                            .short('o')
                            .long("os")
                            .help("os type")
                            .takes_value(true),
                    )
                    .arg(
                        Arg::new("arch")
                            .short('a')
                            .long("arch")
                            .help("arch type")
                            .takes_value(true),
                    )
                    .arg(
                        Arg::new("package")
                            .short('k')
                            .long("package")
                            .help("package type")
                            .takes_value(true),
                    )
                    .arg(
                        Arg::new("folder")
                            .short('f')
                            .long("folder")
                            .help("target folder")
                            .takes_value(true),
                    )
            },
            execute: action_download,
        }],
    })
}

fn action_download(parent: Option<&dyn Module>, param: &ArgMatches) -> Result<(), anyhow::Error> {
    if let Some(parent) = parent {
        println!("download action in {}", parent.name());
    }
    let platform = match param.value_of("os") {
        Some(os) => Platform::from(os),
        None => current_platform(),
    };

    let arch = match param.value_of("arch") {
        Some(a) => Arch::from(a),
        None => current_arch(),
    };

    let folder = match param.value_of("folder") {
        Some(f) => f.to_string(),
        None => std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned(),
    };

    let pkg = match param.value_of("package") {
        Some(pkg_type) => PackageType::from(pkg_type),
        None => match platform {
            Platform::LINUX => PackageType::DEB,
            Platform::WINDOWS => PackageType::EXE,
            _ => PackageType::UNKNOWN,
        },
    };

    //https://www.nomachine.com/download/linux&id=29&s=Raspberry // https://www.nomachine.com/download/download&id=106&s=Raspberry&hw=Pi2
    let download_id = match platform {
        Platform::LINUX => match arch {
            Arch::X86_64 => match pkg {
                PackageType::RPM => 1, //url := "https://www.nomachine.com/download/download&id=1" // https://download.nomachine.com/download/7.9/Linux/nomachine_7.9.2_1_x86_64.rpm
                PackageType::ARCHIVE => 2, //url := "https://www.nomachine.com/download/download&id=2" // https://download.nomachine.com/download/7.9/Linux/nomachine_7.9.2_1_x86_64.tar.gz
                PackageType::DEB => 4, //url := //url := "https://www.nomachine.com/download/download&id=4" // https://download.nomachine.com/download/7.9/Linux/nomachine_7.9.2_1_amd64.deb
                _ => {
                    return Err(anyhow!(
                        "pkg {} not supported on {} platform",
                        pkg,
                        platform
                    ));
                }
            },
            Arch::X86 => match pkg {
                PackageType::ARCHIVE => 3, //url := "https://www.nomachine.com/download/download&id=3" // https://download.nomachine.com/download/7.9/Linux/nomachine_7.9.2_1_i686.tar.gz
                PackageType::RPM => 5, //url := "https://www.nomachine.com/download/download&id=5" // https://download.nomachine.com/download/7.9/Linux/nomachine_7.9.2_1_i686.rpm
                PackageType::DEB => 6, //url := "https://www.nomachine.com/download/download&id=6" // https://download.nomachine.com/download/7.9/Linux/nomachine_7.9.2_1_i386.deb
                _ => {
                    return Err(anyhow!(
                        "pkg {} not supported on {} platform",
                        pkg,
                        platform
                    ));
                }
            },
            _ => {
                return Err(anyhow!(
                    "arch not supported on {} platform{}",
                    platform,
                    arch.to_string()
                ));
            }
        },
        Platform::MACOS => 7, //url = "https://www.nomachine.com/download/download&id=7" // https://download.nomachine.com/download/7.9/MacOSX/nomachine_7.9.2_1.dmg
        Platform::WINDOWS => 8, //"https://www.nomachine.com/download/download&id=8" // https://download.nomachine.com/download/7.9/Windows/nomachine_7.9.2_1.exe
        _ => {
            return Err(anyhow!("os {} not supported", platform));
        }
    };

    let url = format!(
        "https://www.nomachine.com/download/download&id={}",
        download_id
    );
    println!("url: {}", url);

    let response = reqwest::blocking::get(url);
    let content = match response {
        Ok(r) => {
            if let Ok(body) = r.text() {
                //println!("body = {:?}", body);
                body
            } else {
                return Err(anyhow!("os {} not supported", platform));
            }
        }
        _ => {
            return Err(anyhow!("os {} not supported", platform));
        }
    };

    let target_str = "'https://download.nomachine.com/download/";

    let start = if let Some(index) = content.find(target_str) {
        println!("start:{}", index);
        index
    } else {
        return Err(anyhow!("os {} not supported", platform));
    };

    let end = if let Some(index) = content[start..].find("');\"") {
        index
    } else {
        return Err(anyhow!("os {} not supported", platform));
    };

    let target_url = content[start + 1..start + end].to_string();
    println!("target url: {}", target_url);

    let target_folder = std::path::Path::new(&folder);

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(download_file_to_folder(&target_url, target_folder, true))
}
