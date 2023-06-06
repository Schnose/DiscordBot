use crate::{error, Error, Result, State};
use gokz_rs::{Mode, SteamID};
use poise::serenity_prelude::UserId;
use sqlx::FromRow;

/// A discord user in the database
#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
pub struct UserRow {
	/// The user's account id
	pub discord_id: i64,
	/// The user's last known Discord username
	pub name: String,
	/// The user's [`SteamID`]
	pub steam_id: Option<i32>,
	/// The user's favorite [`Mode`]
	pub mode: Option<i16>,
}

/// A discord user
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
	/// The user's account id
	pub discord_id: UserId,
	/// The user's last known Discord username
	pub name: String,
	/// The user's [`SteamID`]
	pub steam_id: Option<SteamID>,
	/// The user's favorite [`Mode`]
	pub mode: Option<Mode>,
}

impl User {
	/// Parses a [`UserRow`] into a [`User`].
	pub async fn from_row(
		UserRow { discord_id, name, steam_id, mode }: UserRow,
		ctx: &crate::Context<'_>,
	) -> Result<Self> {
		let discord_id = match u64::try_from(discord_id) {
			Ok(discord_id) => discord_id,
			Err(err) => {
				error!(ctx.state(), "Invalid UserID found!\n\t{err:?}");
				return Err(Error::InvalidDBUserID);
			}
		};

		let mut user = User {
			discord_id: UserId(discord_id),
			name,
			steam_id: None,
			mode: None,
		};

		if let Some(steam_id) = steam_id {
			let steam_id = match u32::try_from(steam_id) {
				Ok(steam_id) => steam_id,
				Err(err) => {
					error!(ctx.state(), "Invalid SteamID found!\n\t{err:?}");
					return Err(Error::InvalidDBSteamID);
				}
			};

			let steam_id = SteamID::from_id32(steam_id);

			user.steam_id = Some(steam_id);
		}

		if let Some(mode_id) = mode {
			let mode_id = match u8::try_from(mode_id) {
				Ok(mode_id) => mode_id,
				Err(err) => {
					error!(ctx.state(), "Invalid ModeID found!\n\t{err:?}");
					return Err(Error::InvalidDBMode);
				}
			};

			let mode = match Mode::try_from(mode_id) {
				Ok(mode) => mode,
				Err(err) => {
					error!(ctx.state(), "Invalid ModeID found!\n\t{err:?}");
					return Err(Error::InvalidDBMode);
				}
			};

			user.mode = Some(mode);
		}

		Ok(user)
	}
}
