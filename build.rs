include!("src/cli.rs");
//include!("src/runners.rs");

use clap_complete::{generate_to, shells::*};
use clap::CommandFactory;
use std::env;
macro_rules! gen {
	($sh:ident, $cmd:ident, $out:ident) => {
		generate_to($sh, &mut $cmd, "jlaunch", $out.clone()).unwrap();
	}
}
fn main() {
	let outdir = match env::var_os("OUT_DIR") {
		None => panic!(),
		Some(od) => od
	};
	let mut cmd = Args::command();
	gen!(Bash, cmd, outdir);
	gen!(Zsh, cmd, outdir);
	gen!(Fish, cmd, outdir);
}
