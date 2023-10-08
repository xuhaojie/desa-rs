use crate::{BasicAction, BasicActionManager, Module};
use clap::{Arg, ArgMatches, Command};
struct AptModule {
    action_manager: BasicActionManager<Self>,
}

impl Module for AptModule {
    fn name(&self) -> &'static str {
        "apt"
    }

    fn command(&self) -> Command<'static> {
        Command::new(self.name())
            .about("setup apt")
            .arg(
                Arg::new("action")
                    .help("Sets the action to perform")
                    .required(true),
            )
            .arg(
                Arg::new("proxy")
                    .short('p')
                    .long("proxy")
                    .help("Sets a custom proxy")
                    .takes_value(true),
            )
            .arg(
                Arg::new("debug")
                    .short('d')
                    .help("print debug information verbosely"),
            )
    }

    fn execute(&self, param: &ArgMatches) -> std::io::Result<()> {
        if let Some(action) = param.value_of("action") {
            return self.action_manager.execute_action(action, self, param);
        };
        Ok(())
    }
}

pub fn new() -> Box<dyn Module> {
    let module = AptModule {
        action_manager: BasicActionManager {
            actions: vec![
                BasicAction {
                    name: "test",
                    cmd: || {
                        Command::new("download")
                            .about("controls testing features")
                            .version("1.3")
                            .author("Someone E. <someone_else@other.com>")
                            .arg(
                                Arg::with_name("debug")
                                    .short('d')
                                    .help("print debug information verbosely"),
                            )
                    },
                    execute: action_test,
                },
                BasicAction {
                    name: "setup",
                    cmd: || {
                        Command::new("download")
                            .about("controls testing features")
                            .version("1.3")
                            .author("Someone E. <someone_else@other.com>")
                            .arg(
                                Arg::with_name("debug")
                                    .short('d')
                                    .help("print debug information verbosely"),
                            )
                    },
                    execute: action_setup,
                },
            ],
        },
    };
    Box::new(module)
}

fn action_test(module: &AptModule, param: &ArgMatches) -> std::io::Result<()> {
    println!("test action in {}", module.name());
    Ok(())
}

fn action_setup(module: &AptModule, param: &ArgMatches) -> std::io::Result<()> {
    println!("setup action in {}", module.name());
    Ok(())
}
