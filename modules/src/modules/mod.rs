use clap::{Command, ArgMatches};
use anyhow::anyhow;
pub mod apt;
pub mod cargo;
pub mod docker;
pub mod git;
pub mod go;
pub mod nomachine;
pub mod npm;
pub mod pip;
pub mod vmware;
pub mod vscode;


pub trait Module {
    fn name(&self) -> &'static str;
    fn command(&self) -> Command<'static>;
    fn execute(
        &self,
        parent: Option<Box<dyn Module>>,
        param: &ArgMatches,
    ) -> Result<(), anyhow::Error>;
}

pub struct BasicAction {
    name: &'static str,
    cmd: fn() -> Command<'static>,
    execute: fn(param: &ArgMatches) -> Result<(), anyhow::Error>,
}

struct BaseModule {
    name: &'static str,
    description: &'static str,
    actions: Vec<BasicAction>,
}

impl Module for BaseModule {
    fn name(&self) -> &'static str {
        &self.name
    }
    fn command(&self) -> Command<'static> {
        let mut cmd = Command::new(self.name()).about(self.description);
        for action in &self.actions {
            cmd = cmd.subcommand((action.cmd)());
        }
        cmd
    }

    fn execute(
        &self,
        _parent: Option<Box<dyn Module>>,
        param: &ArgMatches,
    ) -> Result<(), anyhow::Error> {
        if let Some(action) = param.subcommand() {
            for act in &self.actions {
                if act.name == action.0 {
                    if let Some(param) = param.subcommand_matches(act.name) {
                        return (act.execute)(param);
                    }
                }
            }
        };

        let _ = self.command().print_help();
        return Err(anyhow!("Please specify sub command for '{}.'", self.name()));
    }
}
