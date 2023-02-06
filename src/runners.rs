use serde::Deserialize;

#[derive(PartialEq, Debug, Deserialize, Clone)]
pub enum Runner {
	Wine,
	WineGE,
	Proton,
	ProtonGE,
	Native,
}
