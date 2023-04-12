use {
	crate::error::Error,
	gokz_rs::{Mode, SteamID},
	sqlx::FromRow,
};

#[derive(Debug, FromRow)]
pub struct UserRow {
	name: String,
	discord_id: i64,
	steam_id: Option<i32>,
	mode: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct User {
	pub name: String,
	pub discord_id: u64,
	pub steam_id: Option<SteamID>,
	pub mode: Option<Mode>,
}

impl TryFrom<UserRow> for User {
	type Error = Error;

	fn try_from(row: UserRow) -> Result<Self, Self::Error> {
		let steam_id = row
			.steam_id
			.map(|id32| SteamID::from_id32(id32 as u32));
		let mode = match row.mode {
			None => None,
			Some(mode) => {
				let mode_id = u8::try_from(mode)?;
				Some(Mode::try_from(mode_id)?)
			}
		};

		Ok(Self {
			name: row.name,
			discord_id: row.discord_id as u64,
			steam_id,
			mode,
		})
	}
}
