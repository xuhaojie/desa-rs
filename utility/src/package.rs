use std::fmt;

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