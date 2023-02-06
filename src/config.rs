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

#[derive(Deserialize, Debug, Clone)]
pub struct ExtraOptions {
	gamescope: bool,
	#[serde(default = "default_gamescope_path")]
	gamescope_path: String,
	gamemode: bool,
	#[serde(default = "default_gamemode_path")]
	gamemode_path: String,
}
impl Default for ExtraOptions {
	fn default() -> ExtraOptions {
		ExtraOptions {
			gamescope: false,
			gamescope_path: default_gamescope_path(),
			gamemode: false,
			gamemode_path: default_gamemode_path(),
		}
	}
}

#[derive(Deserialize, Debug)]
struct Config {
	default_runner: Runner,
	global_options: ExtraOptions,
}
#[derive(Deserialize, Debug, Clone)]
pub struct AppEntry{
	pub game_name: String,
	pub game_path: String,
	pub runner: Runner,
	#[serde(default)]
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


fn get_app(path: &Path) -> AppEntry {
	let file = fs::File::open(path).unwrap();
	serde_json::from_reader(file).unwrap()
}

pub fn get_apps() -> Vec<AppEntry> {
	let xdg_dirs = xdg::BaseDirectories::with_prefix("jlaunch").unwrap();
	let dir = xdg_dirs.get_data_home()
													 .as_path()
													 .to_str().unwrap()
													 .to_string() +
													 "games";
	fs::read_dir(dir).unwrap()
		.map(|x| x.unwrap())
		.filter(|x| x.file_type().unwrap().is_file())
		.map(|x| get_app(x.path().as_path()))
		.collect()
}
