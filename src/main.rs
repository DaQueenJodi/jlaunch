pub mod config;
pub mod runners;
pub mod download;
use std::path::Path;
use clap::{Subcommand, Parser, ValueEnum};
use crate::runners::Runner;
use crate::config::AppEntry;
use crate::config::get_apps;
use std::process;

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
	Native,
}

impl Into<Runner> for DownloadRunner {
	fn into(self) -> Runner {
		match self {
			DownloadRunner::WineGE => Runner::WineGE,
			DownloadRunner::ProtonGE => Runner::ProtonGE,
			DownloadRunner::Native => Runner::Native,
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
		#[arg(short, long, default_value_t = false)]
		gamescope: bool,
		#[arg(short, long, default_value_t = false)]
		gamemode: bool,
		#[arg(short, long, required = false)]
		runner: DownloadRunner,
	}
}

fn main() {
	let args = Args::parse();
	match args.action {
		Action::Download { runner }  => {
			download::download_runner(runner.into());
		},
	  Action::Run { game: name } => {
			let entries: Vec<AppEntry> = get_apps();
			let entry = entries.clone().into_iter().find(|x| x.game_name == name)
				.unwrap_or_else(|| {
					println!("invalid name: {name}");
					println!("valid names: {entries:#?}");
					process::exit(0);
			});
			entry.run();
		},
		Action::Add { name, path, gamescope, runner } =>
	}
}
