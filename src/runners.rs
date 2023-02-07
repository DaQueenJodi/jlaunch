use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Deserialize, Serialize, Clone)]
pub enum Runner {
	Wine,
	WineGE,
	Proton,
	ProtonGE,
	Native,
}
