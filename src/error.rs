use {
	crate::{state::State, target::Target},
	poise::FrameworkError,
	std::num::TryFromIntError,
	thiserror::Error,
	tracing::{debug, error, trace, warn},
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Error, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Error {
	#[error("Some unknown error occurred.")]
	Unknown,

	#[error("{0}")]
	Custom(String),

	#[error("Failed to parse JSON.")]
	Json,

	#[error("`{input}` is not a valid input. Input must be between `{min}` and `{max}`.")]
	OutOfRange { input: u64, min: u64, max: u64 },

	#[error("Failed parsing database row. (`{col}`)")]
	BadDbRow { col: String },

	#[error("{error}")]
	GOKZ { error: gokz_rs::Error },

	#[error("User `{user}` is not in the database.")]
	UserNotInDatabase { user: Target },

	#[error("Expected {expected} but got nothing.")]
	EmptyInput { expected: String },

	#[error("`{input}` is not a global map.")]
	MapNotGlobal { input: String },

	#[error("User does not have a mode preference set. Please use `/mode` to save a mode preference or specify one.")]
	NoModePreference,

	#[error("No records found.")]
	NoRecords,

	#[error("No database entries found.")]
	NoDatabaseEntries,

	#[error("Failed to access database.")]
	DatabaseAccess,
}

impl Error {
	#[tracing::instrument(skip(error))]
	pub async fn handle(error: FrameworkError<'_, State, crate::error::Error>) {
		warn!("Slash Command failed.");
		debug!("{error:?}");

		let (content, ephemeral) = match &error {
			poise::FrameworkError::Command { error, .. } => (error.to_string(), false),
			poise::FrameworkError::ArgumentParse { input, .. } => (
				format!(
					"You provided invalid input. {}",
					if let Some(input) = input { input } else { "" }
				),
				false,
			),
			poise::FrameworkError::CommandStructureMismatch { description, .. } => {
				error!("{description}");
				(String::from("Incorrect command structure."), false)
			}
			poise::FrameworkError::CooldownHit { remaining_cooldown, .. } => {
				(
					format!(
						"This command is currently on cooldown. Please wait another {:.2} seconds before trying again.", remaining_cooldown.as_secs_f64()
					),
					true
				)
			}
			poise::FrameworkError::MissingBotPermissions { missing_permissions, .. } => {
				error!("{missing_permissions}");
				(
					String::from("The bot is missing permissions for this action. Please contact the server owner and kindly ask them to give the bot the required permissions."),
					false
				)
			}
			poise::FrameworkError::MissingUserPermissions { missing_permissions, .. } => {
				(
					if let Some(perms) = missing_permissions {
						format!("You are missing the `{perms}` permissions for this command.")
					} else {
						String::from("You are missing the required permissions for this command.")
					},
					true
				)
			}
			poise::FrameworkError::NotAnOwner { .. } => {
				(String::from("This command requires you to be the owner of the bot."), true)
			}
			why => {
				error!("{why:?}");
				(String::from("Failed to execute command."), true)
			}
		};

		if let Some(ctx) = &error.ctx() {
			if let Err(why) = ctx
				.send(|reply| {
					reply
						.ephemeral(ephemeral)
						.content(&content)
				})
				.await
			{
				error!("Failed to respond to slash command: {why:?}");
			}

			trace!("Handled error with `{content}`.");
		}
	}
}

impl From<serenity::Error> for Error {
	fn from(error: serenity::Error) -> Self {
		match error {
			serenity::Error::Json(why) => {
				error!("Failed to parse JSON.");
				debug!("{why:?}");
				Self::Json
			}
			serenity::Error::NotInRange(param, value, min, max) => {
				warn!("Input for `{param}` was out of range.");
				debug!("Param: `{param}`, Value: `{value}`, Min: `{min}`, Max: `{max}`");
				Self::OutOfRange { input: value, min, max }
			}
			error => {
				warn!("Serenity error occurred: {error:?}");
				Self::Unknown
			}
		}
	}
}

impl From<TryFromIntError> for Error {
	fn from(error: TryFromIntError) -> Self {
		error!("Failed to parse integer.");
		debug!("{error:?}");
		Self::Custom(String::from("Failed to parse integer."))
	}
}

impl From<gokz_rs::Error> for Error {
	fn from(error: gokz_rs::Error) -> Self {
		error!("GOKZ Error.");
		debug!("{error:?}");
		Self::GOKZ { error }
	}
}

impl From<sqlx::Error> for Error {
	fn from(error: sqlx::Error) -> Self {
		error!("SQLx Error.");
		debug!("{error:?}");

		match error {
			sqlx::Error::Database(why) => {
				debug!("{why:?}");
				Self::DatabaseAccess
			}
			sqlx::Error::RowNotFound => Self::NoDatabaseEntries,
			why => {
				debug!("{why:?}");
				Self::Unknown
			}
		}
	}
}
