use {gokz_rs::Tier, poise::ChoiceParameter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, ChoiceParameter)]
pub enum TierChoice {
	#[name = "VeryEasy"]
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

impl From<TierChoice> for Tier {
	fn from(choice: TierChoice) -> Self {
		match choice {
			TierChoice::VeryEasy => Tier::VeryEasy,
			TierChoice::Easy => Tier::Easy,
			TierChoice::Medium => Tier::Medium,
			TierChoice::Hard => Tier::Hard,
			TierChoice::VeryHard => Tier::VeryHard,
			TierChoice::Extreme => Tier::Extreme,
			TierChoice::Death => Tier::Death,
		}
	}
}
