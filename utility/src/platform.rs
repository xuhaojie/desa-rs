use std::fmt;

pub enum Platform{
	LINUX,
	MACOS,
	IOS,
	FREEBSD,
	DRAGONFLY,
	NETBSD,
	OPENBSD,
	SOLARIS,
	ANDROID,
	WINDOWS,
	UNKNOWN,
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Platform::LINUX => write!(f, "linux"),
            Platform::MACOS => write!(f, "macos"),
            Platform::IOS => write!(f, "ios"),
            Platform::FREEBSD => write!(f, "freebsd"),
            Platform::DRAGONFLY => write!(f, "dragonfly"),
            Platform::NETBSD => write!(f, "netbsd"),
            Platform::OPENBSD => write!(f, "openbsd"),
            Platform::SOLARIS => write!(f, "solaris"),
			Platform::ANDROID => write!(f, "android"),
            Platform::WINDOWS => write!(f, "windows"),
            Platform::UNKNOWN => write!(f, "unknown"),			
        }
    }
}

impl From<&str> for Platform{  
    fn from(platform: &str) -> Self {  
        match platform {  
            platform if platform == "linux" => Platform::LINUX,  
			platform if platform == "macos" => Platform::MACOS,  
			platform if platform == "ios" => Platform::IOS,  
		    platform if platform == "freebsd" => Platform::FREEBSD, 
            platform if platform == "dragonfly" => Platform::DRAGONFLY,  
			platform if platform == "netbsd" => Platform::NETBSD,  
			platform if platform == "openbsd" => Platform::OPENBSD,  
		    platform if platform == "solaris" => Platform::SOLARIS,
			platform if platform == "android" => Platform::ANDROID, 
			platform if platform == "windows" => Platform::WINDOWS, 
		    platform if platform == "unknown" => Platform::UNKNOWN, 
			_ => Platform::UNKNOWN,  
		 }  
    }  
}  

pub fn current_platform() -> Platform{
	Platform::from(::std::env::consts::OS)
}
