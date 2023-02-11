use serde::{Deserialize, Serialize};
use crate::cli::{AddRunner, DownloadRunner};

#[derive(PartialEq, Debug, Deserialize, Serialize, Clone)]
pub enum Runner {
	Wine,
	WineGE,
	Terminal,
	Native,
}
impl Into<Runner> for AddRunner {
	fn into(self) -> Runner {
		match self {
			AddRunner::WineGE => Runner::WineGE,
			AddRunner::Terminal => Runner::Terminal,
			AddRunner::Wine => Runner::Wine,
			AddRunner::Native => Runner::Native
		}
	}
}

impl Into<Runner> for DownloadRunner {
	fn into(self) -> Runner {
		match self {
			DownloadRunner::WineGE => Runner::WineGE,
			DownloadRunner::Wine => Runner::Wine
		}
	}
}
