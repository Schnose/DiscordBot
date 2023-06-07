use crate::{Context, Error, Result, State};
use poise::ChoiceParameter;
use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ChoiceParameter)]
pub enum Mode {
	#[name = "KZTimer"]
	KZTimer = 200,

	#[name = "SimpleKZ"]
	SimpleKZ = 201,

	#[name = "Vanilla"]
	Vanilla = 202,
}

impl Mode {
	pub async fn parse_param(mode: Option<Self>, ctx: &Context<'_>) -> Result<gokz_rs::Mode> {
		if let Some(mode) = mode {
			return Ok(mode.into());
		}

		if let Some(user) = ctx
			.get_user_by_id(ctx.author().id)
			.await?
		{
			if let Some(mode) = user.mode {
				return Ok(mode);
			}
		}

		Err(Error::NoMode)
	}
}

impl From<gokz_rs::Mode> for Mode {
	fn from(mode: gokz_rs::Mode) -> Self {
		match mode {
			gokz_rs::Mode::KZTimer => Self::KZTimer,
			gokz_rs::Mode::SimpleKZ => Self::SimpleKZ,
			gokz_rs::Mode::Vanilla => Self::Vanilla,
		}
	}
}

impl From<Mode> for gokz_rs::Mode {
	fn from(mode: Mode) -> Self {
		match mode {
			Mode::KZTimer => gokz_rs::Mode::KZTimer,
			Mode::SimpleKZ => gokz_rs::Mode::SimpleKZ,
			Mode::Vanilla => gokz_rs::Mode::Vanilla,
		}
	}
}

impl Deref for Mode {
	type Target = gokz_rs::Mode;
	fn deref(&self) -> &Self::Target {
		match self {
			Mode::KZTimer => &gokz_rs::Mode::KZTimer,
			Mode::SimpleKZ => &gokz_rs::Mode::SimpleKZ,
			Mode::Vanilla => &gokz_rs::Mode::Vanilla,
		}
	}
}
