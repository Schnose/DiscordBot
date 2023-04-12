use {gokz_rs::Mode, poise::ChoiceParameter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, ChoiceParameter)]
pub enum ModeChoice {
	#[name = "KZTimer"]
	KZTimer = 200,

	#[name = "SimpleKZ"]
	SimpleKZ = 201,

	#[name = "Vanilla"]
	Vanilla = 202,
}

impl From<ModeChoice> for Mode {
	fn from(choice: ModeChoice) -> Self {
		match choice {
			ModeChoice::KZTimer => Mode::KZTimer,
			ModeChoice::SimpleKZ => Mode::SimpleKZ,
			ModeChoice::Vanilla => Mode::Vanilla,
		}
	}
}
