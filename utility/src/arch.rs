use std::fmt;

pub enum Arch{
	X86,
	X86_64,
	ARM,
	AARCH64,
	M68K,
	MIPS,
	MIPS64,
	POWERPC,
	POWERPC64,
	RISCV64,
	S390X,
	SPARC64,
	UNKNOWN,
}

impl fmt::Display for Arch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
			Arch::X86 => write!(f, "x86"),
			Arch::X86_64 => write!(f, "x86_64"),
			Arch::ARM => write!(f, "arm"),
			Arch::AARCH64 => write!(f, "aarch64"),
			Arch::M68K => write!(f, "m68k"),
			Arch::MIPS => write!(f, "mips"),
			Arch::MIPS64 => write!(f, "mips64"),
			Arch::POWERPC => write!(f, "powerpc"),
			Arch::POWERPC64 => write!(f, "powerpc64"),
			Arch::RISCV64 => write!(f, "riscv64"),
			Arch::S390X => write!(f, "s390x"),
			Arch::SPARC64 => write!(f, "sparc64"),
			Arch::UNKNOWN => write!(f, "unknown"),
        }
    }
}

impl From<&str> for Arch{  
    fn from(arch: &str) -> Self {  
        match arch {  
            arch if arch == "x86" => Arch::X86,  
			arch if arch == "x86_64" => Arch::X86_64,  
			arch if arch == "arm" => Arch::ARM,  
		    arch if arch == "aarch64" => Arch::AARCH64,  
            arch if arch == "m68k" => Arch::M68K,  
			arch if arch == "mips" => Arch::MIPS,  
			arch if arch == "mips64" => Arch::MIPS64,  
		    arch if arch == "powerpc" => Arch::POWERPC,  
            arch if arch == "powerpc64" => Arch::POWERPC64,  
			arch if arch == "riscv64" => Arch::RISCV64,  
			arch if arch == "s390x" => Arch::S390X,  
		    arch if arch == "sparc64" => Arch::SPARC64,  		
			arch if arch == "unknown" => Arch::UNKNOWN,  		
			_ => Arch::UNKNOWN,  
		 }  
    }  
}  

pub fn current_arch() -> Arch{
	Arch::from(::std::env::consts::ARCH)
}
