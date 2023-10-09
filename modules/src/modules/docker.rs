use crate::{BaseModule, BasicAction, Module};
use clap::{Arg, ArgMatches, Command};

pub fn new() -> Box<dyn Module> {
    Box::new(BaseModule {
        name: "docker",
        description: "Setup docker",
        actions: vec![BasicAction {
            name: "proxy",
            cmd: || {
                Command::new("proxy").about("setup docker proxy").arg(
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

fn action_setup(_parent: Option<&dyn Module>, param: &ArgMatches) -> std::io::Result<()> {
    if let Some(parent) = _parent {
        println!("setup action in {}", parent.name());
    }

    if let Some(_action) = param.value_of("proxy") {
        let config = param.value_of("proxy").unwrap_or("default.conf");
        println!("Value for proxy: {}", config);
    }
    Ok(())
}
