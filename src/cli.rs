use clap::{Subcommand, Parser, ValueEnum};
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
#[arg(short, long, default_value_t = false)]
	quiet: bool,

#[command(subcommand)]
	pub action: Action,
}
#[derive(Parser, Debug, ValueEnum, PartialEq, Eq, Clone)]
pub enum DownloadRunner {
	WineGE,
	Wine,
}

#[derive(ValueEnum, Debug, Clone, PartialEq, Eq)]
pub enum AddRunner  {
	Wine,
	WineGE,
	Native,
	Terminal
}

#[derive(Subcommand, Debug)]
pub enum Action {
	Download {
		runner: DownloadRunner
	},
	Run {
		name: String
	},
	Add {
		name: String,
		path: String,
		#[arg(long, default_value_t = false)]
		gamescope: bool,
		#[arg(long, default_value_t = false)]
		gamemode: bool,
		#[arg(short, long, value_enum, default_value_t = AddRunner::Native)]
		runner: AddRunner
	},
	List,
	Remove {
		name: String
	}
}

