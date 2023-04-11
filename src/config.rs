use {serde::Deserialize, std::collections::HashSet};

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
	/// The environment the bot is currently running in. This will determine whether slash commands
	/// are registered on a single guild or globally. It will also determine which Discord API Token
	/// to use when authenticating, as well as which Database URL.
	pub environment: Environment,

	/// Steam WebAPI token
	pub steam_token: String,

	/// List of `UserID`s with special privileges
	pub owners: HashSet<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
pub enum Environment {
	Development {
		/// Discord API Token
		discord_token: String,

		/// PostgreSQL connection string
		database_url: String,

		/// PostgreSQL table name to store user information in
		users_table: String,

		/// `ChannelID` to send reports to
		report_channel_id: u64,

		/// `GuildID` for registering slash commands
		guild_id: u64,
	},
	Production {
		/// Discord API Token
		discord_token: String,

		/// PostgreSQL connection string
		database_url: String,

		/// PostgreSQL table name to store user information in
		users_table: String,

		/// `ChannelID` to send reports to
		report_channel_id: u64,
	},
}

impl std::fmt::Display for Environment {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(match self {
			Environment::Development { .. } => "DEV",
			Environment::Production { .. } => "PROD",
		})
	}
}
