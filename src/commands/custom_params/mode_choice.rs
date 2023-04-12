use {
	crate::{
		state::{Context, StateContainer},
		target::Target,
	},
	gokz_rs::Mode,
	poise::ChoiceParameter,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, ChoiceParameter)]
pub enum ModeChoice {
	#[name = "KZTimer"]
	KZTimer = 200,

	#[name = "SimpleKZ"]
	SimpleKZ = 201,

	#[name = "Vanilla"]
	Vanilla = 202,
}

impl ModeChoice {
	/// Figure out a mode to use if the user did not specify one.
	pub async fn figure_out(target: Target, ctx: &Context<'_>) -> Mode {
		ctx.fetch_user(target.clone())
			.await
			.and_then(|user| user.mode)
			.unwrap_or(Mode::KZTimer)
	}
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
