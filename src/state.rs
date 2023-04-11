use {
	crate::{
		config::{self, Config},
		error::Error,
	},
	schnosebot::global_map::GlobalMap,
	sqlx::{postgres::PgPoolOptions, Pool, Postgres},
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
			.connect(match &config.environment {
				config::Environment::Development { database_url, .. } => database_url,
				config::Environment::Production { database_url, .. } => database_url,
			})
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

pub trait StateContainer {
	fn schnose(&self) -> &str;
	fn icon_url(&self) -> &str;
	fn color(&self) -> (u8, u8, u8);
	fn gokz_client(&self) -> &gokz_rs::Client;
	fn db(&self) -> &Pool<Postgres>;
	fn maps(&self) -> &[GlobalMap];
	fn map_names(&self) -> &[String];
}

impl StateContainer for Context<'_> {
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
}