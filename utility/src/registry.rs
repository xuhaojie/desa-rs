use anyhow::anyhow;
use clap::ArgMatches;

pub struct Registry<'a> {
    pub name: &'a str,
    pub caption: &'a str,
    pub url: &'a str,
}

pub fn list_registers(registers :&[Registry]) {
    for r in registers {
        print!("{} [{}] \n{}\n", r.caption, r.name, r.url);
    }
}

pub fn set_registry(registrys:&[Registry], target:&str, action: fn(&Registry)->  Result<(), anyhow::Error>)  -> Result<(), anyhow::Error> {
	let mut index: i32 = -1;
	let mut i = 0;
	for r in registrys.iter() {
		if r.name == target {
			index = i;
			break;
		}
		i += 1;
	}
	if index < 0 {
		println!("available mirror is:");
		list_registers(registrys);
		return Err(anyhow!("invalid mirror"));
	}
	else {
		let registry = &registrys[index as usize];
		action(registry)?;
		println!("set proxy to {} succeeded", registry.name);
	} 
	Ok(())
}

pub fn setup_proxy_action(
    param: &ArgMatches,
	registrys:&[Registry],
	action: fn(&Registry)->  Result<(), anyhow::Error>
) -> Result<(), anyhow::Error> {
    if let Some(mirror) = param.value_of("mirror") {
		set_registry(registrys,mirror,action)
    } else {
		println!("available mirror is:");
		list_registers(registrys);
        Err(anyhow!("miss param for mirror"))
    }
}
