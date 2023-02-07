pub mod config;
pub mod runners;
pub mod download;
use std::path::Path;
use clap::{Subcommand, Parser, ValueEnum};
use crate::config::ExtraOptions;
use crate::runners::Runner;
use crate::config::AppEntry;
use crate::config::get_apps;
use std::process;
use std::fs;
use std::io::Write;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
#[arg(short, long, default_value_t = false)]
quiet: bool,

#[command(subcommand)]
action: Action,
}
#[derive(Parser, Debug, ValueEnum, PartialEq, Eq, Clone)]
enum DownloadRunner {
WineGE,
ProtonGE,
}

#[derive(Parser, Debug, ValueEnum, PartialEq, Eq, Clone)]
enum AddRunner  {
WineGE,
ProtonGE,
Native,
}

impl Into<Runner> for AddRunner {
fn into(self) -> Runner {
	match self {
		AddRunner::WineGE => Runner::WineGE,
		AddRunner::ProtonGE => Runner::WineGE,
		AddRunner::Native => Runner::Native,
	}
}
}

impl Into<Runner> for DownloadRunner {
fn into(self) -> Runner {
	match self {
		DownloadRunner::WineGE => Runner::WineGE,
		DownloadRunner::ProtonGE => Runner::ProtonGE,
	}
}
}

#[derive(Subcommand, Debug)]
enum Action {
Download {
	runner: DownloadRunner
},
Run {
	game: String,
},
Add {
	name: String,
	path: String,
	#[arg(long, default_value_t = false)]
	gamescope: bool,
	#[arg(long, default_value_t = false)]
	gamemode: bool,
	#[arg(short, long)]
	runner: AddRunner,
},
List,
}

fn main() {
let args = Args::parse();
match args.action {
	Action::Download { runner }  => {
		download::download_runner(runner.into());
	},
	Action::Run { game: name } => {
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
		}
	}
}
