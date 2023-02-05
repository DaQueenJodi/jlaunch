use std::process::{Command, ExitStatus};
use serde_derive::Deserialize;
use std::path::Path;
use std::vec::Vec;

#[derive(PartialEq, Debug, Deserialize)]
pub enum Runner {
	Wine,
	WineGE,
	Proton,
	ProtonGE,
	Native,
}

const PROTON_PATH: &'static str = "./proton.sh";
pub fn run(runner: Runner, path: String) -> ExitStatus {
	let mut args = Vec::new();
	match runner {
		Runner::Proton => args.push(PROTON_PATH),
		Runner::Native => (),
		_ => todo!()
	}
	args.push(&path);
	let mut cmd = Command::new(args[0]);
	cmd.args(&args[1..]);
	cmd.status().unwrap()
}
