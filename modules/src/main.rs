pub mod modules;
use crate::modules::*;
use clap::Command;

//#[tokio::main]
fn main() {
    //utility::download::download_package("https://www.vmware.com/go/getfusion").await;
    //return;
    let modules: Vec<Box<dyn Module>> = vec![
        apt::new(),
        cargo::new(),
        docker::new(),
        git::new(),
        go::new(),
        pip::new(),
        // nomachine::new(), // 下载NoMachine功能不正常，怀疑官方的下载路径做了调整，先关闭这项功能
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

    let matches = cmd.clone().get_matches();

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
        let _ = cmd.print_help();
    }
}

// desa git setup -e xuhaojie@hotmail.com -u xuhaojie
