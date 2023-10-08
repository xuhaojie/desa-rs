use crate::{BaseModule, BasicAction, Module};
use clap::{Arg, ArgMatches, Command};

pub fn new() -> Box<dyn Module> {
    Box::new(BaseModule {
        name: "apt",
        description: "Setup apt mirror",
        actions: vec![
			BasicAction {
            name: "mirror",
            cmd: || {
                Command::new("mirror").about("setup apt mirror").arg(
                    Arg::new("proxy")
                        .short('p')
                        .long("proxy")
                        .help("setup a custom proxy")
                        .takes_value(true),
                )
            },
            execute: action_setup,
        }],
    })
}

fn action_setup(parent: Option<&dyn Module>, param: &ArgMatches) -> std::io::Result<()> {
    if let Some(parent) = parent {
        println!("setup action in {}", parent.name());
    }

    Ok(())
}
