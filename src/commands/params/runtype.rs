use poise::ChoiceParameter;
use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ChoiceParameter)]
pub enum Runtype {
	#[name = "TP"]
	TP = 1,

	#[name = "PRO"]
	Pro = 0,
}

impl From<bool> for Runtype {
	fn from(b: bool) -> Self {
		match b {
			true => Self::TP,
			false => Self::Pro,
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
			Runtype::TP => &true,
			Runtype::Pro => &false,
		}
	}
}

impl Default for Runtype {
	fn default() -> Self {
		Self::Pro
	}
}
