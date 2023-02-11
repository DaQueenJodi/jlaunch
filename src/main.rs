pub mod config;
pub mod runners;
pub mod download;
pub mod cli;
use crate::cli::Action;
use clap::Parser;
use crate::cli::Args;
use std::path::Path;
use crate::config::{ExtraOptions, get_apps, remove_app, AppEntry};
use crate::runners::Runner;
use std::process;
use std::fs;
use std::io::Write;

fn main() {
	let args = Args::parse();
	match args.action {
		Action::Download { runner }  => {
			download::download_runner(runner.into());
		},
		Action::Run { name } => {
			let entries: Vec<AppEntry> = get_apps();
			let entry = entries.clone().into_iter().find(|x| x.name == name)
				.unwrap_or_else(|| {
					println!("invalid name: {name}");
					process::exit(1);
			});
			entry.run();
		},
		Action::List => {
			let entries: Vec<AppEntry> = get_apps();
			for entry in entries {
				println!("{}", entry.name);
			}
		},
		Action::Add { name, path, gamescope, gamemode, runner } => {
			if get_apps().into_iter().any(|e| e.name == name) {
				println!("Sorry, there is already an entry for this game, please choose a different name");
				process::exit(1);
			}
			// check if the path is valid and also expand it
			// TODO: make this expand '~'
			let path = Path::new(&path);
			if !path.exists() {
				println!("this file does not exist!");
				process::exit(1);
			}
			if !path.is_file() {
				println!("this is not a file");
				process::exit(1);
			}
			// turn it into canonical (absolute) path
			let path = path.canonicalize().unwrap()
				.as_os_str()
				.to_str().unwrap()
				.to_string();

			let entry = AppEntry {
				name,
				path,
				runner: runner.into(),
				options: ExtraOptions {
					gamescope,
					gamemode,
				}
			};
			let ser = serde_json::to_string(&entry).unwrap();
			let xdg_dirs = xdg::BaseDirectories::with_prefix("jlaunch").unwrap();
			let games_path = xdg_dirs.get_data_home().as_path().to_str().unwrap().to_string();
			let base_path_str = games_path + "/games/" + &entry.name;
			let mut path_str: String;
			let mut path = Path::new(&base_path_str);
			{
				let mut suffix = 1;
				while path.exists() {
					path_str = base_path_str.clone() + &suffix.to_string();
					path = Path::new(&path_str);
					suffix += 1;
				}
			}

			let mut file = fs::File::create(path).unwrap();
			file.write_all(ser.as_bytes()).unwrap();
			println!("successfully added {}", entry.name);
		}
		Action::Remove { name } => {
			remove_app(name.clone()).unwrap_or_else(|_| {
				println!("invalid name: {name}");
				process::exit(1);
			});
			println!("successfully removed {name}");
		}
	}
}
