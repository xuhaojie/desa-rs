pub mod modules;
use crate::modules::*;
use clap::{ArgMatches, Command};
use std::io;

pub trait Module {
    fn name(&self) -> &'static str;
    fn command(&self) -> Command<'static>;
    //fn register<'a>(&self, app : App<'a>) -> App<'a>;
    fn execute(&self, parent: Option<Box<dyn Module>>, param: &ArgMatches) -> std::io::Result<()>;
}

pub struct BasicAction {
    name: &'static str,
    cmd: fn() -> Command<'static>,
    execute: fn(parent: Option<&dyn Module>, param: &ArgMatches) -> std::io::Result<()>,
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
        let cmd = Command::new(self.name()).about(self.description);
        let mut c: Command = cmd;
        for action in &self.actions {
            c = c.subcommand((action.cmd)());
        }
        c
    }

    fn execute(&self, _parent: Option<Box<dyn Module>>, param: &ArgMatches) -> std::io::Result<()> {
        if let Some(action) = param.subcommand() {
            for act in &self.actions {
                if act.name == action.0 {
                    if let Some(param) = param.subcommand_matches(act.name) {
                        return (act.execute)(Some(self as &dyn Module), param);
                    }
                }
            }
        };
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("require sub command for '{}'", self.name()),
        ));
    }
}

fn main() -> std::io::Result<()> {
    let modules: Vec<Box<dyn Module>> = vec![
        // apt::new(), // 功能尚不能用
        cargo::new(),
        // docker::new(), // 功能尚不能用
        git::new(),
        go::new(),
        pip::new(),
        nomachine::new(),
        npm::new(),
        vmware::new(),
        vscode::new(),
    ];

    let mut cmd = Command::new("desa")
        .version("1.0")
        .author("Xu Haojie <xuhaojie@hotmail.com>")
        .about("Development enveriment setup assist");

    for module in &modules {
        cmd = cmd.subcommand(module.command());
    }

    let matches = cmd.get_matches();
    /*
        // 如果用户提供、则获取该值作为config，或者默认使用 “default.conf”
        let config = matches.value_of("config").unwrap_or("default.conf");
        println!("Value for config: {}", config);

        // 在这里调用.unwrap（）是安全的，因为需要“ INPUT”（如果不需要“ INPUT”，
        // 可以使用 “if let” 有条件地获取值）

        if let Some(input) = matches.value_of("INPUT"){
            println!("Using input file: {}", input);
        }
    */
    // 根据用户使用“详细”标志的次数来改变输出
    // (比如 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    /*
    match matches.occurrences_of("v") {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }
    */

    // 你可以通过以下方式处理有关子命令的信息：按名称请求它们的匹配（如下所示）
    // 仅请求正在使用的名称或两者同时请求
    let mut found = false;
    for module in &modules {
        if let Some(matches) = matches.subcommand_matches(module.name()) {
            found = true;
            //println!("execute module {}", module.name());
            let result = module.execute(None, matches);
            if let Err(e) = result {
                println!("{}", e.to_string());
            }

            break;
        }
    }
    if !found {
        println!("please specify a command, for more infomation please type desa --help");
    }
    Ok(())
}

// desa git setup -e xuhaojie@hotmail.com -u xuhaojie
