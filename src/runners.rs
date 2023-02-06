use serde::Deserialize;

#[derive(PartialEq, Debug, Deserialize)]
pub enum Runner {
	Wine,
	WineGE,
	Proton,
	ProtonGE,
	Native,
}
