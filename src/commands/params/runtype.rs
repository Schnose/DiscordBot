use poise::ChoiceParameter;
use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ChoiceParameter)]
pub enum Runtype {
	#[name = "PRO"]
	Pro = 0,

	#[name = "TP"]
	TP = 1,
}

impl From<bool> for Runtype {
	fn from(b: bool) -> Self {
		match b {
			false => Self::Pro,
			true => Self::TP,
		}
	}
}

impl From<Runtype> for bool {
	fn from(b: Runtype) -> Self {
		matches!(b, Runtype::TP)
	}
}

impl Deref for Runtype {
	type Target = bool;
	fn deref(&self) -> &Self::Target {
		match self {
			Runtype::Pro => &false,
			Runtype::TP => &true,
		}
	}
}

impl Default for Runtype {
	fn default() -> Self {
		Self::Pro
	}
}
