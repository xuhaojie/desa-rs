use crate::{Module , BasicAction,BasicActionManager};
use clap::{Arg, ArgMatches, Command};
use dirs;
use std::io;

struct CargoModule{
	action_manager: BasicActionManager<Self>,
}

impl Module for CargoModule{

	fn name(&self) -> &'static str{
		"cargo"
	}

	fn command<'a>(&self) -> Command<'a> {
		Command::new(self.name())
		.about("setup cargo")
		.arg(Arg::new("action")
			.help("Sets the action to perform")
			.required(true))
		.arg(Arg::new("mirror")
			.short('m')
			.long("mirror")
			.help("Set mirror name")
			.takes_value(true))
		.arg(Arg::new("debug")
			.short('d')
			.help("print debug information verbosely"))
	}

	fn execute(&self, param: &ArgMatches) -> std::io::Result<()> {
		if let Some(action) = param.value_of("action"){
			return self.action_manager.execute_action(action, self, param);
		};
		Ok(())
	}
}

pub fn new() -> Box<dyn Module> {
	Box::new(CargoModule{
		action_manager: BasicActionManager{
			actions:vec![
				BasicAction{name:"test",  execute: action_test},
				BasicAction{name:"setup", execute: action_setup},
			]
		}
	})
}

fn action_test(module: &CargoModule, param:&ArgMatches)  -> std::io::Result<()>{
	println!("test action in {}", module.name());
	Ok(())
}

fn action_setup(module: &CargoModule, param:&ArgMatches) -> std::io::Result<()>{
	println!("setup action in {}", module.name());
/*
	let mut lines = vec![
		"[source.crates-io]",
		"registry =\"https://github.com/rust-lang/crates.io-index\"",
		"# 指定镜像"
		"replace-with = '镜像源名'"
		"# 中国科学技术大学"
		"[source.ustc]registry = \"https://mirrors.ustc.edu.cn/crates.io-index\""
		# 上海交通大学
		[source.sjtu]registry = "https://mirrors.sjtug.sjtu.edu.cn/git/crates.io-index/" 
		# 清华大学
		[source.tuna]registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"
		# rustcc社区
		[source.rustcc]registry = "https://code.aliyun.com/rustcc/crates.io-index.git"
		"registry = \"https://mirrors.ustc.edu.cn/crates.io-index\"",
	}

	];
*/
	//# 如：tuna、sjtu、ustc，或者 rustcc

	let home_dir = match dirs::home_dir() {
		Some(path) => path,
		None => return Err(io::Error::new(io::ErrorKind::Other,"can't get home dir")),
	};

	let target_path = home_dir.join(".cargo");
	let target_file = target_path.join("config");
	if !target_path.exists(){
		std::fs::create_dir(target_path);
	}
	if target_file.exists(){
		std::fs::remove_file(target_file.as_path());
	}
 /* 
	if let Some(mirror) = param.value_of("mirror"){
		lines[4] = &format!("registry = \"{}\"", mirror);
	}
*/





	Ok(())
}


/*

func SetupCargoProxy(mirror string) error {
	lines := []string{
		"[source.crates-io]",
		"registry = \"https://github.com/rust-lang/crates.io-index\"",
		"replace-with = 'mirror'",
		"[source.mirror]",
		"registry = \"https://mirrors.ustc.edu.cn/crates.io-index\"",
	}

	lines[4] = fmt.Sprintf("registry = \"%s\"", mirror)

	cfgSize := 0
	for _, line := range lines {
		cfgSize += len(line) + 1
	}
	cfg_data := make([]byte, 0, cfgSize)

	for _, line := range lines {
		cfg_data = append(cfg_data, line...)
		cfg_data = append(cfg_data, '\n')
	}

	userHomeDir, err := os.UserHomeDir()
	if err != nil {
		return err
	}

	targetPath := path.Join(userHomeDir, ".cargo")
	targetFile := path.Join(targetPath, "config")
	exist, err := common.PathExists(targetPath)
	if err != nil {
		return err
	}
	if !exist {
		err := os.Mkdir(targetPath, 0664)
		if err != nil {
			return err
		}
	}

	err = ioutil.WriteFile(targetFile, []byte(cfg_data), os.FileMode(0644))
	return err
}

*/