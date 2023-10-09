use crate::{BaseModule, BasicAction, Module};
use clap::{Arg, ArgMatches, Command};
use std::fmt;
use std::io;
use utility::{arch::*, download::*, execute::*, package::*, platform::*};

pub enum BuildType {
    STABLE,
    INSIDER,
    UNKNOWN,
}

impl fmt::Display for BuildType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BuildType::STABLE => write!(f, "stable"),
            BuildType::INSIDER => write!(f, "insider"),
            BuildType::UNKNOWN => write!(f, "unknown"),
        }
    }
}

impl From<&str> for BuildType {
    fn from(build_type: &str) -> Self {
        match build_type {
            build_type if build_type == "stable" => BuildType::STABLE,
            build_type if build_type == "insider" => BuildType::INSIDER,
            build_type if build_type == "unknown" => BuildType::UNKNOWN,
            _ => BuildType::UNKNOWN,
        }
    }
}

pub fn new() -> Box<dyn Module> {
    Box::new(BaseModule {
        name: "vscode",
        description: "Download vscode",
        actions: vec![BasicAction {
            name: "download",
            cmd: || {
                Command::new("download")
                    .about("download vscode")
                    .arg(
                        Arg::new("proxy")
                            .short('p')
                            .long("proxy")
                            .help("Sets a custom proxy")
                            .takes_value(true),
                    )
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
                    .arg(
                        Arg::with_name("debug")
                            .short('d')
                            .help("print debug information verbosely"),
                    )
            },
            execute: action_download,
        }],
    })
}

fn gen_download_url(
    base: &str,
    build: &BuildType,
    os: Platform,
    arch: Arch,
    pkg: PackageType,
) -> std::io::Result<String> {
    let result = match os {
        Platform::WINDOWS => {
            let os_str = "win32";
            let arch_str = match arch {
                Arch::X86 => "",
                Arch::X86_64 => "x64",
                Arch::AARCH64 => "arm64",
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!("arch not supported on {} platform{}", os, arch.to_string()),
                    ))
                }
            };

            match pkg {
                PackageType::EXE | PackageType::MSI | PackageType::UNKNOWN => {
                    if arch_str.len() > 0 {
                        format!("{}?build={}&os={}-{}", base, build, os_str, arch_str)
                    } else {
                        format!("{}?build={}&os={}", base, build, os_str)
                    }
                }
                PackageType::ARCHIVE => {
                    if arch_str.len() > 0 {
                        format!(
                            "{}?build={}&os={}-{}-{}",
                            base, build, os_str, arch_str, "archive"
                        )
                    } else {
                        format!("{}?build={}&os={}-{}", base, build, os_str, "archive")
                    }
                }
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!("package type not supported on {} platform {}", os, pkg),
                    ))
                }
            }
        }

        Platform::LINUX => {
            let os_str = "linux";
            let arch_str = match arch {
                Arch::X86_64 => "x64",
                Arch::ARM => "armhf",
                Arch::AARCH64 => "arm64",
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!("arch not supported on {} platform {}", os, arch.to_string()),
                    ))
                }
            };

            match pkg {
                PackageType::DEB | PackageType::RPM => format!(
                    "{}?build={}&os={}-{}-{}",
                    base, build, os_str, pkg, arch_str
                ),
                PackageType::ARCHIVE => format!("{}?build={}&os={}-{}", base, build, os, arch_str),
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!("package type not supported on {} platform {}", os, pkg),
                    ))
                }
            }
        }

        Platform::MACOS => {
            let os_str = "darwin";
            let arch_str = match arch {
                Arch::X86_64 => "",
                Arch::AARCH64 => "arm64",
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!("arch not supported on {} platform {}", os, arch.to_string()),
                    ))
                }
            };

            if arch_str.len() > 0 {
                format!("{}?build={}&os={}-{}", base, build, os_str, arch_str)
            } else {
                format!("{}?build={}&os={}", base, build, os_str)
            }
        }
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("not supported platform {}", os),
            ));
        }
    };

    return Ok(result.to_string());
}

fn replace_vscode_download_url(url: &str, build: BuildType, newbase: &str) -> String {
    // newbase = "https://vscode.cdn.azure.cn"
    //https: //vscode.cdn.azure.cn/stable/b4c1bd0a9b03c749ea011b06c6d2676c8091a70c/VSCodeUserSetup-x64-1.57.0.exe

    println!("url:{}", url);
    let target_str = format!("/{}/", build);
    if let Some(index) = url.find(&target_str) {
        format!("{}{}", newbase, &url[index..])
    } else {
        url.to_string()
    }
}

fn action_download(parent: Option<&dyn Module>, param: &ArgMatches) -> std::io::Result<()> {
    let build = BuildType::STABLE;

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

    let download_url = gen_download_url(
        "https://code.visualstudio.com/sha/download",
        &build,
        platform,
        arch,
        pkg,
    )?;

    let redirected_url = utility::download::get_redirected_url(&download_url)?;

    let final_url =
        replace_vscode_download_url(&redirected_url, build, "https://vscode.cdn.azure.cn");
    println!("final_url: {}", final_url);
    let target_folder = std::path::Path::new(&folder);
    download_file(&final_url, target_folder, true)
}
