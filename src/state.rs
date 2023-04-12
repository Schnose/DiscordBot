use {
	crate::{config::Config, database, error::Error, target::Target},
	gokz_rs::{MapIdentifier, Mode, SteamID},
	poise::async_trait,
	schnosebot::global_map::GlobalMap,
	sqlx::{postgres::PgPoolOptions, Pool, Postgres, QueryBuilder},
	tracing::error,
};

pub type Context<'ctx> = poise::Context<'ctx, State, Error>;

/// Global state object that gets passed to event && command handlers.
#[derive(Debug, Clone)]
pub struct State {
	/// Parsed config file for the bot
	pub config: Config,

	/// (͡ ͡° ͜ つ ͡͡°)
	pub schnose: String,

	/// (͡ ͡° ͜ つ ͡͡°)
	pub icon_url: String,

	/// #7480C2 - the best color in existence
	pub color: (u8, u8, u8),

	/// HTTP Client to make API requests
	pub gokz_client: gokz_rs::Client,

	/// Postgres connection pool for storing user data
	pub database_connection: Pool<Postgres>,

	/// Cache of all global maps
	pub global_maps: Vec<GlobalMap>,

	/// Cache of all global map names
	pub global_maps_names: Vec<String>,
}

impl State {
	pub async fn new(config: Config) -> Self {
		let schnose = String::from("(͡ ͡° ͜ つ ͡͡°)");
		let icon_url = String::from(
			"https://media.discordapp.net/attachments/981130651094900756/1068608508645347408/schnose.png"
		);
		let color = (116, 128, 194);
		let gokz_client = gokz_rs::Client::new();

		let database_connection = PgPoolOptions::new()
			.min_connections(5)
			.max_connections(20)
			.connect(&config.database_url)
			.await
			.expect("Failed to connect to database.");

		let global_maps = GlobalMap::fetch(true, &gokz_client)
			.await
			.expect("Failed to fetch global maps.");

		let global_maps_names = global_maps
			.iter()
			.map(|map| map.name.clone())
			.collect();

		Self {
			config,
			schnose,
			icon_url,
			color,
			gokz_client,
			database_connection,
			global_maps,
			global_maps_names,
		}
	}
}

#[async_trait]
pub trait StateContainer {
	fn config(&self) -> &Config;
	fn schnose(&self) -> &str;
	fn icon_url(&self) -> &str;
	fn color(&self) -> (u8, u8, u8);
	fn gokz_client(&self) -> &gokz_rs::Client;
	fn db(&self) -> &Pool<Postgres>;
	fn maps(&self) -> &[GlobalMap];
	fn map_names(&self) -> &[String];
	fn get_map(&self, map_identifier: impl Into<MapIdentifier>) -> Option<GlobalMap>;

	fn author_id(&self) -> u64;

	async fn fetch_user(&self, target: Target) -> Option<database::User> {
		match target {
			Target::None { user_id } | Target::Mention { user_id } => {
				self.fetch_user_by_id(user_id).await
			}
			Target::SteamID { steam_id } => {
				self.fetch_user_by_steam_id(steam_id)
					.await
			}
			Target::Name { name } => self.fetch_user_by_name(&name).await,
		}
	}
	async fn fetch_user_by_id(&self, discord_id: u64) -> Option<database::User>;
	async fn fetch_user_by_name(&self, username: &str) -> Option<database::User>;
	async fn fetch_user_by_steam_id(&self, steam_id: SteamID) -> Option<database::User>;
	async fn fetch_user_by_mode(&self, mode: Mode) -> Option<database::User>;
}

#[async_trait]
impl StateContainer for Context<'_> {
	fn config(&self) -> &Config {
		&self.data().config
	}

	fn schnose(&self) -> &str {
		&self.data().schnose
	}

	fn icon_url(&self) -> &str {
		&self.data().icon_url
	}

	fn color(&self) -> (u8, u8, u8) {
		self.data().color
	}

	fn gokz_client(&self) -> &gokz_rs::Client {
		&self.data().gokz_client
	}

	fn db(&self) -> &Pool<Postgres> {
		&self.data().database_connection
	}

	fn maps(&self) -> &[GlobalMap] {
		&self.data().global_maps
	}

	fn map_names(&self) -> &[String] {
		&self.data().global_maps_names
	}

	fn get_map(&self, map_identifier: impl Into<MapIdentifier>) -> Option<GlobalMap> {
		GlobalMap::fuzzy_search(self.maps(), map_identifier)
	}

	fn author_id(&self) -> u64 {
		*self.author().id.as_u64()
	}

	async fn fetch_user_by_id(&self, discord_id: u64) -> Option<database::User> {
		let table_name = &self.config().database_url;

		match sqlx::query_as::<_, database::UserRow>(&format!(
			"SELECT * FROM {table_name} WHERE discord_id = {discord_id}"
		))
		.fetch_optional(self.db())
		.await
		{
			Ok(row) => match row?.try_into() {
				Ok(user) => Some(user),
				Err(why) => {
					error!("Failed to parse user: {why:?}");
					None
				}
			},
			Err(why) => {
				error!("Failed to fetch user from DB: {why:?}");
				None
			}
		}
	}

	async fn fetch_user_by_name(&self, username: &str) -> Option<database::User> {
		let table_name = &self.config().database_url;

		let mut query = QueryBuilder::new(format!("SELECT * FROM {table_name}"));
		query
			.push(" WHERE name LIKE ")
			.push_bind(format!("%{username}%"));

		match query
			.build_query_as::<database::UserRow>()
			.fetch_optional(self.db())
			.await
		{
			Ok(row) => match row?.try_into() {
				Ok(user) => Some(user),
				Err(why) => {
					error!("Failed to parse user: {why:?}");
					None
				}
			},
			Err(why) => {
				error!("Failed to fetch user from DB: {why:?}");
				None
			}
		}
	}

	async fn fetch_user_by_steam_id(&self, steam_id: SteamID) -> Option<database::User> {
		let table_name = &self.config().database_url;

		match sqlx::query_as::<_, database::UserRow>(&format!(
			"SELECT * FROM {table_name} WHERE steam_id = {}",
			steam_id.as_id32()
		))
		.fetch_optional(self.db())
		.await
		{
			Ok(row) => match row?.try_into() {
				Ok(user) => Some(user),
				Err(why) => {
					error!("Failed to parse user: {why:?}");
					None
				}
			},
			Err(why) => {
				error!("Failed to fetch user from DB: {why:?}");
				None
			}
		}
	}

	async fn fetch_user_by_mode(&self, mode: Mode) -> Option<database::User> {
		let table_name = &self.config().database_url;

		match sqlx::query_as::<_, database::UserRow>(&format!(
			"SELECT * FROM {table_name} WHERE mode = {}",
			mode as u8
		))
		.fetch_optional(self.db())
		.await
		{
			Ok(row) => match row?.try_into() {
				Ok(user) => Some(user),
				Err(why) => {
					error!("Failed to parse user: {why:?}");
					None
				}
			},
			Err(why) => {
				error!("Failed to fetch user from DB: {why:?}");
				None
			}
		}
	}
}
