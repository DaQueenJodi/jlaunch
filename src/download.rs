use crate::runners::Runner;
use serde_json::Value;
use std::str;
use std::fs;
use std::process::Command;

fn download_file(url: String) -> Vec<u8> {
	todo!()
}

fn get_latest_gh_release(repo: &str) -> Vec<String>  {
	let url = "https://api.github.com/repos/".to_string() +
					 repo +
					 "/releases/latest";
	let json = ureq::get(&url)
		.call().unwrap()
		.into_string().unwrap();
	let json: Value = serde_json::from_str(&json).unwrap();
	let mut url_vec = Vec::new();
	for asset in json["assets"].as_array().unwrap() {
		url_vec.push(asset["browser_download_url"].as_str().unwrap().to_string());
	}
	return url_vec;
}


pub fn download_runner(runner: Runner) {
	match runner {
	 Runner::WineGE => {
		 let urls = get_latest_gh_release("GloriousEggroll/wine-ge-custom");
		 let url = urls.into_iter().find(|s| s.ends_with(".tar.xz")).unwrap();
		 let mut cmd = Command::new("scripts/dl-wine-ge.sh");
		 let xdg_dirs = xdg::BaseDirectories::with_prefix("jlaunch").unwrap();
		 cmd
			 .arg(url)
			 .arg(xdg_dirs.get_data_home().as_path().to_str().unwrap().to_string() + "/runners")
			 .status()
			 .expect("failed to download wine-ge");
	 },
		_ => panic!("todo: right now only downloading wine-ge is implemented, sorry")
	}
}
