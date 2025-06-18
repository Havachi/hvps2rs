
use serde::Serialize;
use serde::Deserialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Region {
	Japan,
	NorthAmerica,
	Europ,
}