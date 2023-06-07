use poise::ChoiceParameter;
use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ChoiceParameter)]
pub enum Tier {
	#[name = "Very Easy"]
	VeryEasy = 1,

	#[name = "Easy"]
	Easy = 2,

	#[name = "Medium"]
	Medium = 3,

	#[name = "Hard"]
	Hard = 4,

	#[name = "VeryHard"]
	VeryHard = 5,

	#[name = "Extreme"]
	Extreme = 6,

	#[name = "Death"]
	Death = 7,
}

impl From<gokz_rs::Tier> for Tier {
	fn from(tier: gokz_rs::Tier) -> Self {
		match tier {
			gokz_rs::Tier::VeryEasy => Self::VeryEasy,
			gokz_rs::Tier::Easy => Self::Easy,
			gokz_rs::Tier::Medium => Self::Medium,
			gokz_rs::Tier::Hard => Self::Hard,
			gokz_rs::Tier::VeryHard => Self::VeryHard,
			gokz_rs::Tier::Extreme => Self::Extreme,
			gokz_rs::Tier::Death => Self::Death,
		}
	}
}

impl From<Tier> for gokz_rs::Tier {
	fn from(tier: Tier) -> Self {
		match tier {
			Tier::VeryEasy => gokz_rs::Tier::VeryEasy,
			Tier::Easy => gokz_rs::Tier::Easy,
			Tier::Medium => gokz_rs::Tier::Medium,
			Tier::Hard => gokz_rs::Tier::Hard,
			Tier::VeryHard => gokz_rs::Tier::VeryHard,
			Tier::Extreme => gokz_rs::Tier::Extreme,
			Tier::Death => gokz_rs::Tier::Death,
		}
	}
}

impl Deref for Tier {
	type Target = gokz_rs::Tier;
	fn deref(&self) -> &Self::Target {
		match self {
			Tier::VeryEasy => &gokz_rs::Tier::VeryEasy,
			Tier::Easy => &gokz_rs::Tier::Easy,
			Tier::Medium => &gokz_rs::Tier::Medium,
			Tier::Hard => &gokz_rs::Tier::Hard,
			Tier::VeryHard => &gokz_rs::Tier::VeryHard,
			Tier::Extreme => &gokz_rs::Tier::Extreme,
			Tier::Death => &gokz_rs::Tier::Death,
		}
	}
}
