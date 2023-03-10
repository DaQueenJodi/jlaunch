use serde::{Deserialize, Serialize};
use std::process::{Command, ExitStatus};
use crate::runners::Runner;
use std::path::Path;
use std::fs;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ExtraOptions {
	pub gamescope: bool,
	pub gamemode: bool,
}
impl Default for ExtraOptions {
	fn default() -> ExtraOptions {
		ExtraOptions {
			gamescope: false,
			gamemode: false,
		}
	}
}

#[derive(Deserialize, Debug)]
struct Config {
	pub default_runner: Runner,
	pub global_options: ExtraOptions,
}
#[derive(Deserialize, Serialize,  Debug, Clone)]
pub struct AppEntry{
	pub name: String,
	pub path: String,
	pub runner: Runner,
	#[serde(default)]
	pub options: ExtraOptions,
}

const GAMEMODE_PATH: &'static str = "/usr/bin/gamemoderun";
const GAMESCOPE_PATH: &'static str = "/usr/bin/gamescope";
const WINE_GE_PATH: &'static str = "/wine-ge/bin/wine";
const TERMINAL: &'static str = "alacritty";

impl AppEntry {
	pub fn run(&self) -> ExitStatus {
		let xdg_dirs = xdg::BaseDirectories::with_prefix("jlaunch").unwrap();
		let data_home = xdg_dirs.get_data_home().as_path().to_str().unwrap().to_string();
		let runner_path = data_home.clone() + "runners";
		let prefix_path = data_home.clone() + "prefixes";
		let mut args = Vec::new();
		let options = &self.options; 
		if options.gamescope {
			args.push(GAMESCOPE_PATH);
		}
		if options.gamemode {
			args.push(GAMEMODE_PATH);
		}
		let path;
		match self.runner {
			Runner::Native => (),
			Runner::WineGE => {
				path = runner_path + WINE_GE_PATH;
				args.push(&path);
				let prefix = format!("{prefix_path}/{}", self.name);
				std::env::set_var("WINEPREFIX", &prefix);
				fs::create_dir_all(&prefix).unwrap();
			},
			Runner::Terminal => {
				args.push(TERMINAL);
				args.push("-e");
			}
			_ => todo!()
		}
		args.push(&self.path);
		let mut cmd = Command::new(args[0]);
		cmd.args(&args[1..]);
		let parent = Path::new(&self.path).parent().unwrap();
		println!("setting working directory to {parent:?}..");
		std::env::set_current_dir(parent).unwrap();
		// set current wine prefix
		println!("done, launching game!");
		println!("{args:?}");
		cmd.status().unwrap()
	}
}


fn get_app(path: &Path) -> AppEntry {
	let file = fs::File::open(path).unwrap();
	serde_json::from_reader(file).unwrap()
}
fn get_app_files() -> Vec<String> {
	let xdg_dirs = xdg::BaseDirectories::with_prefix("jlaunch").unwrap();
	let dir = xdg_dirs.get_data_home()
													 .as_path()
													 .to_str().unwrap()
													 .to_string() +
													 "games";
	fs::read_dir(dir).unwrap()
		.map(|f| f.unwrap())
		.filter(|f| f.file_type().unwrap().is_file())
		.map(|f| f.path()
				 .as_path()
				 .to_str()
				 .unwrap()
				 .to_string())
		.collect()
}
pub fn get_apps() -> Vec<AppEntry> {
	get_app_files()
		.into_iter()
		.map(|s| get_app(Path::new(&s)))
		.collect()
}

pub fn remove_app(name: String) -> Result<(), String> {
	let files = get_app_files();
	let pos = files.clone()
		.into_iter()
		.map(|s| get_app(Path::new(&s)))
		.position(|e| e.name == name);
	match pos {
		Some(n) => fs::remove_file(&files[n]).unwrap(),
		None => return Err("could not find the app entry".to_string()),
	}
	Ok(())
}
