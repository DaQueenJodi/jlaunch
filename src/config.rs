use serde::Deserialize;
use std::process::{Command, ExitStatus};
use crate::runners::Runner;
use std::path::Path;
use std::fs;

fn default_gamescope_path() -> String {
	String::from("/usr/bin/gamescope")
}

fn default_gamemode_path() -> String {
	String::from("/usr/bin/gamemoderun")
}


#[derive(Deserialize, Debug)]
pub struct ExtraOptions {
	gamescope: bool,
	#[serde(default = "default_gamescope_path")]
	gamescope_path: String,
	gamemode: bool,
	#[serde(default = "default_gamemode_path")]
	gamemode_path: String,
}

#[derive(Deserialize, Debug)]
struct Config {
	default_runner: Runner,
	global_options: ExtraOptions,
}
#[derive(Deserialize, Debug)]
pub struct AppEntry{
	pub game_name: String,
	pub game_path: String,
	pub runner: Runner,
	pub options: ExtraOptions,
}

impl AppEntry {
	pub fn run(&self) -> ExitStatus {
		let xdg_dirs = xdg::BaseDirectories::with_prefix("jlaunch").unwrap();
		let runner_path = xdg_dirs.get_data_home().as_path().to_str().unwrap().to_string() + "runners";
		let mut args = Vec::new();
		let options = &self.options; 
		if options.gamescope {
			args.push(options.gamescope_path.as_str());
		}
		if options.gamemode {
			args.push(&options.gamemode_path.as_str());
		}
		let path;
		match self.runner {
			Runner::Native => (),
			Runner::WineGE => {
				path = runner_path + "/wine-ge/bin/wine";
				args.push(&path);
			},
			_ => todo!()
		}
		args.push(&self.game_path);
		let mut cmd = Command::new(args[0]);
		cmd.args(&args[1..]);
		let parent = Path::new(&self.game_path).parent().unwrap();
		println!("setting working directory to {parent:?}..");
		std::env::set_current_dir(parent);
		println!("done, launching game!");
		println!("{args:?}");
		cmd.status().unwrap()
	}
}


pub fn get_app(path: &Path) -> AppEntry {
	let file = fs::File::open(path).unwrap();
	serde_json::from_reader(file).unwrap()
}
