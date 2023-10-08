use crate::{BasicAction, BaseModule, Module};
use clap::{Arg, ArgMatches, Command};
use utility::execute::{self, Cmd};

pub fn new() -> Box<dyn Module> {
    Box::new(BaseModule {
		name: "git",
		description:"Setup git",
		actions: vec![
			BasicAction {
				name: "setup",
				cmd: || {
					Command::new("setup").about("setup git")
					.arg(
						Arg::new("user")
							.short('u')
							.long("user")
							.help("Sets user name")
							.takes_value(true),
					)
					.arg(
						Arg::new("email")
							.short('e')
							.long("email")
							.help("Sets email address")
							.takes_value(true),
					)
				},
				execute: action_setup,
			},
        ],
    })
}


fn action_setup(parent: Option<&dyn Module>, param: &ArgMatches) -> std::io::Result<()> {
    if let Some(user) = param.value_of("user") {
        let mut cmd = Cmd {
            cmd: "git",
            params: vec!["config", "--global", "user.name"],
        };
        //let cmd = format!("git config --global user.name {}", user);
        cmd.params.push(user);
        if let Ok(code) = execute::execute_command(&cmd) {
            if 0 == code {
                println!("exec {} succeed", cmd.to_string());
            }
        }
    }

    if let Some(email) = param.value_of("email") {
        let mut cmd = Cmd {
            cmd: "git",
            params: vec!["config", "--global", "user.email"],
        };
        //let cmd = format!("git config --global user.name {}", user);
        cmd.params.push(email);
        if let Ok(code) = execute::execute_command(&cmd) {
            if 0 == code {
                println!("exec {} succeed", cmd.to_string());
            }
        }
    }
    Ok(())
}
