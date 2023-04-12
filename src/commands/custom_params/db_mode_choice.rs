use {gokz_rs::Mode, poise::ChoiceParameter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, ChoiceParameter)]
pub enum DBModeChoice {
	#[name = "None"]
	None = 0,

	#[name = "KZTimer"]
	KZTimer = 200,

	#[name = "SimpleKZ"]
	SimpleKZ = 201,

	#[name = "Vanilla"]
	Vanilla = 202,
}

impl From<DBModeChoice> for Option<Mode> {
	fn from(choice: DBModeChoice) -> Self {
		match choice {
			DBModeChoice::None => None,
			DBModeChoice::KZTimer => Some(Mode::KZTimer),
			DBModeChoice::SimpleKZ => Some(Mode::SimpleKZ),
			DBModeChoice::Vanilla => Some(Mode::Vanilla),
		}
	}
}
