use serde_derive::Deserialize;
use crate::runners::Runner;
use std::path::Path;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct ExtraOptions {
	gamescope: bool,
	gamemode: bool,
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

pub fn get_app(path: &Path) -> AppEntry {
	let file = fs::File::open(path).unwrap();
	serde_json::from_reader(file).unwrap()
}
