use shuttle_secrets::SecretStore;

#[derive(Debug, Clone)]
pub struct Config {
	/// Discord API Token
	pub discord_token: String,

	/// PostgreSQL connection string
	pub database_url: String,

	/// PostgreSQL table name to store user information in
	pub users_table: String,

	/// `ChannelID` to send reports to
	pub report_channel_id: u64,

	/// `GuildID` for registering slash commands
	pub guild_id: Option<u64>,

	/// Steam WebAPI token
	pub steam_token: String,

	/// `UserID` with special privileges
	pub owner_id: u64,
}

impl Config {
	pub fn new(secret_store: &SecretStore) -> Self {
		Self {
			discord_token: secret_store
				.get("DISCORD_TOKEN")
				.expect("Missing `DISCORD_TOKEN` secret."),
			database_url: secret_store
				.get("DATABASE_URL")
				.expect("Missing `DATABASE_URL` secret."),
			users_table: secret_store
				.get("USERS_TABLE")
				.expect("Missing `USERS_TABLE` secret."),
			report_channel_id: secret_store
				.get("REPORT_CHANNEL_ID")
				.expect("Missing `REPORT_CHANNEL_ID` secret.")
				.parse()
				.expect("`REPORT_CHANNEL_ID` must be a u64."),
			guild_id: secret_store
				.get("GUILD_ID")
				.map(|guild_id| {
					guild_id
						.parse()
						.expect("`GUILD_ID` must be a u64.")
				}),
			steam_token: secret_store
				.get("STEAM_TOKEN")
				.expect("Missing `STEAM_TOKEN` secret."),
			owner_id: secret_store
				.get("OWNER_ID")
				.expect("Missing `OWNER_ID` secret.")
				.parse()
				.expect("`OWNER_ID` must be a u64."),
		}
	}
}
