pub mod config;
pub mod runners;
pub mod download;
use std::path::Path;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
	
}

enum Action {
	
}


fn main() {
  let path = Path::new("welp.json");
  let app =  config::get_app(path);
	app.run();
	//download::download_runner(runners::Runner::WineGE);
}
