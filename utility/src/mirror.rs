use anyhow::anyhow;
use clap::ArgMatches;

pub struct Mirror<'a> {
    pub name: &'a str,
    pub caption: &'a str,
    pub url: &'a str,
}

impl ToString for Mirror<'_> {
	fn to_string(&self) -> String{
		format!("{} [{}] \n{}\n", self.caption, self.name, self.url)
	}
}

pub fn list_mirrors(mirrors :&[Mirror]) {
    for m in mirrors {
        print!("{}\n", m.to_string());
    }
}

pub fn set_mirror(mirrors:&[Mirror], target:&str, action: fn(&Mirror)->  Result<(), anyhow::Error>)  -> Result<(), anyhow::Error> {
	let mut index: i32 = -1;
	let mut i = 0;
	for m in mirrors.iter() {
		if m.name == target {
			index = i;
			break;
		}
		i += 1;
	}
	if index < 0 {
		println!("available mirror is:");
		list_mirrors(mirrors);
		return Err(anyhow!("invalid mirror"));
	}
	else {
		let mirror = &mirrors[index as usize];
		action(mirror)?;
		println!("set proxy to {} succeeded", mirror.name);
	} 
	Ok(())
}

pub fn setup_mirror_action(
    param: &ArgMatches,
	param_name:&str,
	mirrors:&[Mirror],
	action: fn(&Mirror)->  Result<(), anyhow::Error>
) -> Result<(), anyhow::Error> {
    if let Some(mirror) = param.value_of(param_name) {
		set_mirror(mirrors,mirror,action)
    } else {
		println!("available mirror is:");
		list_mirrors(mirrors);
        Err(anyhow!("miss param for mirror"))
    }
}
