use poise::ChoiceParameter;
use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ChoiceParameter)]
pub enum Bool {
	#[name = "Yes"]
	Yes = 1,

	#[name = "No"]
	No = 0,
}

impl From<bool> for Bool {
	fn from(b: bool) -> Self {
		match b {
			true => Self::Yes,
			false => Self::No,
		}
	}
}

impl From<Bool> for bool {
	fn from(b: Bool) -> Self {
		matches!(b, Bool::Yes)
	}
}

impl Deref for Bool {
	type Target = bool;
	fn deref(&self) -> &Self::Target {
		match self {
			Bool::Yes => &true,
			Bool::No => &false,
		}
	}
}
