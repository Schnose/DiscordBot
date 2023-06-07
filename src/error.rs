use crate::{error, warn, State};
use poise::FrameworkError;
use thiserror::Error;

/// Type alias for convenience.
pub type Result<T> = std::result::Result<T, Error>;

/// Creates an [`Error`] from anything that can be turned into a [`String`].
#[macro_export]
macro_rules! err {
	( $( $args:tt )* ) => {
		$crate::Error::Custom(format!( $( $args )* ))
	};
}

/// Creates an [`Error`] from anything that can be turned into a [`String`] and returns it.
#[macro_export]
macro_rules! yeet {
	($err:expr) => {
		return Err($err);
	};

	( $( $args:tt )* ) => {
		return Err($crate::err!( $( $args )* ));
	};
}

/// The crate's global error type.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum Error {
	/// Custom error for one-off scenarios that don't justify a separate variant.
	#[error("{0}")]
	Custom(String),

	/// A user's ID in the database was not a valid [`u64`]
	#[error("Invalid UserID found. Please report this.")]
	InvalidDBUserID,

	/// A user's SteamID in the database was not valid
	#[error("Invalid SteamID found. Please report this.")]
	InvalidDBSteamID,

	/// A user's Mode in the database was not valid
	#[error("Invalid Mode found. Please report this.")]
	InvalidDBMode,

	/// A log has an invalid ID
	#[error("Invalid LogID found. Please report this.")]
	InvalidLogID,

	/// A log has an invalid level
	#[error("Invalid Log level `{0}` found. Please report this.")]
	InvalidLogLevel(i16),

	/// A user submitted a parameter that was out of bounds
	#[error("The value `{value}` for `{param}` is out of bounds. Please submit a value that is at least `{min}` and at max `{max}`.")]
	OutOfBoundsParam {
		/// The parameter in question
		param: String,
		/// The value submitted by the user
		value: u64,
		/// The lower bound
		min: u64,
		/// The upper bound
		max: u64,
	},
}

impl Error {
	/// Global error handler for slash commands
	pub async fn global_handler(error: FrameworkError<'_, crate::GlobalState, Error>) {
		if let Some(ctx) = error.ctx() {
			error!(ctx.state(), "Received error.");
		} else {
			error!("Received error.");
		}

		match error {
			FrameworkError::Setup { error, framework, .. } => {
				let state = framework.user_data().await;
				error!(state, "## Failed to setup.\n```\n{error:?}\n```");
			}

			FrameworkError::EventHandler { error, framework, .. } => {
				let state = framework.user_data().await;
				error!(state, "## Failed to handle event.\n```\n{error:?}\n```");
			}

			FrameworkError::Command { error, ctx } => {
				let state = ctx.state();
				error!(state, "## Failed to handle command.\n```\n{error:?}\n```");
			}

			FrameworkError::CommandPanic { payload, ctx } => {
				let state = ctx.state();
				error!(state, "## Panicked during command execution.\n```\n{payload:?}\n```");

				if let Err(err) = ctx
					.send(|reply| {
						reply.content("Panicked during command execution. Please report this.")
					})
					.await
				{
					error!("Failed to reply to user.\n```\n{err:?}\n```");
				}
			}

			FrameworkError::ArgumentParse { error, input, ctx } => {
				let state = ctx.state();
				warn!(
					state,
					"## Failed to parse arguments.\n\n### Error:\n```\n{error:?}\n```\n### Input:\n```\n{input:?}\n```"
				);

				if let Err(err) = ctx
					.send(|reply| reply.content(error.to_string()))
					.await
				{
					error!("Failed to reply to user.\n```\n{err:?}\n```");
				}
			}

			FrameworkError::MissingBotPermissions { missing_permissions, ctx } => {
				let state = ctx.state();
				warn!(
					state,
					"## Bot has insufficcient permissions.\n```\n{missing_permissions:?}\n```"
				);

				if let Err(err) = ctx
					.send(|reply| reply.content("Missing permissions <:PogO:824260850701434891>"))
					.await
				{
					error!("Failed to reply to user.\n```\n{err:?}\n```");
				}
			}

			FrameworkError::NotAnOwner { ctx } => {
				if let Err(err) = ctx
					.send(|reply| reply.content("You don't have permissions to use this command."))
					.await
				{
					error!("Failed to reply to user.\n```\n{err:?}\n```");
				}
			}

			FrameworkError::GuildOnly { ctx } => {
				if let Err(err) = ctx
					.send(|reply| reply.content("This command only works in servers."))
					.await
				{
					error!("Failed to reply to user.\n```\n{err:?}\n```");
				}
			}

			error => {
				if let Some(ctx) = error.ctx() {
					error!(ctx.state(), "## Received error.\n```\n{error}\n```");
				} else {
					error!("Received error.\n```\n{error}\n```");
				}
			}
		};
	}
}

impl From<sqlx::Error> for Error {
	fn from(err: sqlx::Error) -> Self {
		if let sqlx::Error::RowNotFound = err {
			return Self::Custom(String::from("Requested database entry not found."));
		}

		Self::Custom(err.to_string())
	}
}

impl From<poise::serenity_prelude::SerenityError> for Error {
	fn from(err: poise::serenity_prelude::SerenityError) -> Self {
		if let poise::serenity_prelude::Error::NotInRange(param, value, min, max) = err {
			return Self::OutOfBoundsParam { param: param.to_owned(), value, min, max };
		}

		Self::Custom(err.to_string())
	}
}
