pub mod config;
pub mod runners;
use std::path::Path;
fn main() {
	let path = Path::new("welp.json");
  let app =  config::get_app(path);
	runners::run(app.runner, app.game_path);
}
