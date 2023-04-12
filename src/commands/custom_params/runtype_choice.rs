use poise::ChoiceParameter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ChoiceParameter)]
pub enum RuntypeChoice {
	#[name = "TP"]
	TP,

	#[name = "PRO"]
	Pro,
}

impl From<RuntypeChoice> for bool {
	fn from(choice: RuntypeChoice) -> Self {
		matches!(choice, RuntypeChoice::TP)
	}
}
