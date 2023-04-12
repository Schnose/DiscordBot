use poise::ChoiceParameter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ChoiceParameter)]
pub enum BoolChoice {
	#[name = "No"]
	No = 0,

	#[name = "Yes"]
	Yes = 1,
}

impl From<BoolChoice> for bool {
	fn from(choice: BoolChoice) -> Self {
		matches!(choice, BoolChoice::Yes)
	}
}
