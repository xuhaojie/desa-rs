use std::io;
use crate::{Module , BasicAction, BasicActionManager};
use clap::{Arg, ArgMatches, Command};
use utility::{download::*, execute::*, platform::*, arch::*};
use std::fmt;

pub enum BuildType{
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

impl From<&str> for BuildType{  
    fn from(build_type: &str) -> Self {  
        match build_type {  
            build_type if build_type == "stable" => BuildType::STABLE,  
			build_type if build_type == "insider" => BuildType::INSIDER,  
			build_type if build_type == "unknown" => BuildType::UNKNOWN,  		
			_ => BuildType::UNKNOWN,  
		 }  
    }  
} 

pub enum PackageType {
	UNKNOWN,
	EXE,
	MSI,
	DEB,
	RPM,
	ARCHIVE
}

impl fmt::Display for PackageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
			PackageType::EXE => write!(f, "exe"),
			PackageType::MSI => write!(f, "msi"),
			PackageType::DEB => write!(f, "deb"),
			PackageType::RPM => write!(f, "rpm"),
			PackageType::ARCHIVE => write!(f, "archive"),
			PackageType::UNKNOWN => write!(f, "unknown"),			
        }
    }
}

impl From<&str> for PackageType{  
    fn from(package_type: &str) -> Self {  
        match package_type {  
            package_type if package_type == "exe" => PackageType::EXE,
			package_type if package_type == "msi" => PackageType::MSI,
			package_type if package_type == "deb" => PackageType::DEB,
            package_type if package_type == "rpm" => PackageType::RPM, 
			package_type if package_type == "archive" => PackageType::ARCHIVE, 
			package_type if package_type == "unknown" => PackageType::UNKNOWN,
			_ => PackageType::UNKNOWN,  
		 }
    } 
}

struct VScodeModule{
	action_manager: BasicActionManager<Self>,
}

impl Module for VScodeModule{

	fn name(&self) -> &'static str{
		"vscode"
	}

	fn command<'a>(&self) -> Command<'a> {
		Command::new(self.name())
		.about("download vscode")
		.arg(Arg::new("action")
			.help("Sets the action to perform")
			.required(true))
		.arg(Arg::new("proxy")
			.short('p')
			.long("proxy")
			.help("Sets a custom proxy")
			.takes_value(true))
		.arg(Arg::new("os")
			.short('o')
			.long("os")
			.help("os type")
			.takes_value(true))				
		.arg(Arg::new("debug")
			.short('d')
			.help("print debug information verbosely"))
	}

	fn execute(&self, param: &ArgMatches) -> std::io::Result<()> {
		if let Some(action) = param.value_of("action"){
			return self.action_manager.execute_action(action, self, param);
		};
		Ok(())
	}
}

pub fn new() -> Box<dyn Module> {
	let module = VScodeModule{
		action_manager: BasicActionManager{
			actions:vec![
				BasicAction{name:"download",  execute: action_download},
				BasicAction{name:"setup", execute: action_setup},
			]
		}
	};
	Box::new(module)
}

fn gen_download_url(build: &BuildType, os: Platform, arch : Arch, pkg : PackageType) -> std::io::Result<String>  {
	let arch = current_arch();
	let os = current_platform();
	let base = "https://code.visualstudio.com/sha/download";
	let result = match os {
		Platform::WINDOWS => {
			let os_str = "win32";
			let arch_str = match arch {
				Arch::X86 => "",
				Arch::X86_64 => "x64",
				Arch::AARCH64 => "arm64",
				_ =>  return Err(io::Error::new(io::ErrorKind::Other,format!("arch not supported on {} platform{}", os, arch.to_string()))),
			};
	
			match pkg {
				PackageType::EXE | PackageType::MSI | PackageType::UNKNOWN => {
					if arch_str.len() > 0 {
						format!("{}?build={}&os={}-{}", base, build, os_str, arch_str)
					} else {
						format!("{}?build={}&os={}", base, build, os_str)
					}
				},
				PackageType::ARCHIVE => {
					if arch_str.len() > 0 {
						format!("{}?build={}&os={}-{}-{}", base, build, os_str, arch_str, "archive")
					} else {
						format!("{}?build={}&os={}-{}", base, build, os_str, "archive")
					}
				},
				_ => return Err(io::Error::new(io::ErrorKind::Other,format!("package type not supported on {} platform {}",os, pkg))),
			}
		},
		
		Platform::LINUX => {
			let os_str = "linux";
			let arch_str = match arch {
				Arch::X86 => "",
				Arch::ARM => "armhf",
				Arch::AARCH64 => "arm64",
				_ =>  return Err(io::Error::new(io::ErrorKind::Other,format!("arch not supported on {} platform{}", os, arch.to_string()))),
			};
	
			match pkg {
				PackageType::DEB | PackageType::RPM => format!("{}?build={}&os={}-{}-{}", base, build, os_str, pkg, arch_str),
				PackageType::ARCHIVE => format!("{}?build={}&os={}-{}", base, build, os, arch_str),
				_ => return Err(io::Error::new(io::ErrorKind::Other,format!("package type not supported on {} platform {}", os, pkg))),
			}
		},
		
		Platform::MACOS => {
			let os_str = "darwin";
			let arch_str = match arch {
				Arch::X86_64 => "",
				Arch::AARCH64 => "arm64",
				_ =>  return Err(io::Error::new(io::ErrorKind::Other,format!("arch not supported on {} platform {}", os, arch.to_string()))),
			};
	
			if arch_str.len() > 0 {
				format!("{}?build={}&os={}-{}", base, build, os_str, arch_str)
			} else {
				format!("{}?build={}&os={}", base, build, os_str)
			}
		},
		_ => {
			return Err(io::Error::new(io::ErrorKind::Other,format!("not supported platform {}", os)));
		}
	};

	return Ok(result.to_string());
}

fn replace_vscode_download_url(url: &str, build : BuildType, newbase : &str) -> String {
	// newbase = "https://vscode.cdn.azure.cn"
	//https: //vscode.cdn.azure.cn/stable/b4c1bd0a9b03c749ea011b06c6d2676c8091a70c/VSCodeUserSetup-x64-1.57.0.exe

	println!("url:{}",url);
	let target_str = format!("/{}/",build);
	if let Some(index) = url.find(&target_str){
		format!("{}{}",newbase, &url[index..])
	} else {
		url.to_string()
	}
}

fn action_download(module: &VScodeModule, param:&ArgMatches) -> std::io::Result<()> {
	println!("download action in {}", module.name());
	let build = BuildType::STABLE;
	let platform = current_platform();
	let arch = current_arch();
	let pkg = PackageType::UNKNOWN;
	
	let download_url = gen_download_url(&build, platform, arch, pkg)?;

	let redirected_url = utility::download::get_redirected_url(&download_url)?;

	let  final_url = replace_vscode_download_url(&redirected_url, build, "https://vscode.cdn.azure.cn");
	println!("final_url: {}", final_url);
	let target_folder = std::path::Path::new("/tmp");
	download_file(&final_url, target_folder, true)
}

fn action_setup(module: &VScodeModule, param:&ArgMatches) -> std::io::Result<()>{
	println!("setup action in {}", module.name());
	if let Some(action) = param.value_of("proxy"){
		let config = param.value_of("proxy").unwrap_or("default.conf");
		println!("Value for proxy: {}", config);
	}
	Ok(())
}
