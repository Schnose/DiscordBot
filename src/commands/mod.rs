mod ping;
pub use ping::ping;

mod db;
pub use db::db;

mod setsteam;
pub use setsteam::setsteam;

mod params {
	use poise::ChoiceParameter;
	use std::ops::Deref;

	#[derive(Debug, Clone, Copy, PartialEq, Eq, ChoiceParameter)]
	pub enum Bool {
		#[name = "No"]
		No = 0,

		#[name = "Yes"]
		Yes = 1,
	}

	impl From<bool> for Bool {
		fn from(b: bool) -> Self {
			match b {
				false => Self::No,
				true => Self::Yes,
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
				Bool::No => &false,
				Bool::Yes => &true,
			}
		}
	}
}
