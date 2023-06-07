use crate::{database, err, info, trace, Context, Result};
use poise::{
	async_trait,
	serenity_prelude::{ChannelId, CreateEmbedFooter, Http, UserId},
};
use schnosebot::global_map::GlobalMap;
use shuttle_service::SecretStore;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, QueryBuilder};
use std::sync::Arc;
use tokio::sync::OnceCell;

/// The bot's global state. This hold configuration options, secrets, and other "global"
/// information the bot should always have access to.
#[derive(Debug)]
pub struct GlobalState {
	/// The bot's owner's Discord UserID
	pub owner_id: UserId,

	/// The channel to which the bot should send logs to
	pub logs_channel: ChannelId,

	/// The channel to which the bot should send reports to
	pub reports_channel: ChannelId,

	/// PostgreSQL database pool
	pub database_pool: Arc<Pool<Postgres>>,

	/// Table name for user information
	pub users_table: String,

	/// Table name for logs
	pub logs_table: String,

	/// Default color for embeds
	pub color: (u8, u8, u8),

	/// Default icon for embeds
	pub icon: String,

	/// HTTP Context for Discord
	pub http: Option<Arc<Http>>,

	/// HTTP Client to make API requests
	pub gokz_client: gokz_rs::Client,

	/// A cache of all global KZ maps
	pub global_maps: &'static Vec<GlobalMap>,
}

/// Global cache of all global KZ maps
pub static GLOBAL_MAPS: OnceCell<Vec<GlobalMap>> = OnceCell::const_new();

/// Global database pool
pub static DATABASE_POOL: OnceCell<Arc<Pool<Postgres>>> = OnceCell::const_new();

impl GlobalState {
	/// Creates a new [`GlobalState`] instance.
	#[tracing::instrument(skip(secret_store))]
	pub async fn new(secret_store: &SecretStore) -> Result<Self> {
		trace!("Setting up global state...");

		let owner_id = secret_store
			.get("OWNER_ID")
			.ok_or(err!("Missing secret `OWNER_ID`"))?
			.parse()
			.map(UserId)
			.map_err(|err| err!("Invalid `OWNER_ID`: {err:?}"))?;

		trace!("Got `owner_id`");

		let logs_channel = secret_store
			.get("LOGS_CHANNEL_ID")
			.ok_or(err!("Missing secret `LOGS_CHANNEL_ID`"))?
			.parse()
			.map(ChannelId)
			.map_err(|err| err!("Invalid `LOGS_CHANNEL_ID`: {err:?}"))?;

		trace!("Got `logs_channel`");

		let reports_channel = secret_store
			.get("REPORTS_CHANNEL_ID")
			.ok_or(err!("Missing secret `REPORTS_CHANNEL_ID`"))?
			.parse()
			.map(ChannelId)
			.map_err(|err| err!("Invalid `REPORTS_CHANNEL_ID`: {err:?}"))?;

		trace!("Got `reports_channel`");

		let database_url = secret_store
			.get("DATABASE_URL")
			.ok_or(err!("Missing secret `DATABASE_URL`"))?;

		trace!("Got `database_url`");

		let database_pool = PgPoolOptions::new()
			.min_connections(10)
			.max_connections(50)
			.connect(&database_url)
			.await?;

		let database_pool = Arc::clone(
			DATABASE_POOL
				.get_or_init(|| async { Arc::new(database_pool) })
				.await,
		);

		info!("Connected to database.");

		let users_table = secret_store
			.get("USERS_TABLE")
			.ok_or(err!("Missing secret `USERS_TABLE`"))?;

		trace!("Got `users_table`");

		let logs_table = secret_store
			.get("LOGS_TABLE")
			.ok_or(err!("Missing secret `LOGS_TABLE`"))?;

		trace!("Got `logs_table`");

		let color = (116, 128, 194);
		let icon = String::from("https://cdn.discordapp.com/attachments/981130651094900756/1068608508645347408/schnose.png");

		let gokz_client = gokz_rs::Client::new();

		trace!("Got gokz client");

		let global_maps = GLOBAL_MAPS
			.get_or_init(|| async {
				schnosebot::global_map::GlobalMap::fetch(true, &gokz_client)
					.await
					.expect("Failed to fetch global maps.")
			})
			.await;

		trace!("Got global maps");

		Ok(Self {
			owner_id,
			logs_channel,
			reports_channel,
			database_pool,
			users_table,
			logs_table,
			color,
			icon,
			http: None,
			gokz_client,
			global_maps,
		})
	}
}

/// Extension trait for [`poise::Context`] so I can call custom methods on it to access
/// [`GlobalState`].
#[allow(missing_docs)]
#[async_trait]
pub trait State {
	fn state(&self) -> &GlobalState;
	fn owner(&self) -> UserId;
	fn logs_channel(&self) -> ChannelId;
	fn db(&self) -> &Pool<Postgres>;
	fn logs_table(&self) -> &str;
	fn users_table(&self) -> &str;

	/// Get the default color used for embeds
	fn color(&self) -> (u8, u8, u8);

	/// (͡ ͡° ͜ つ ͡͡°)
	fn icon(&self) -> &String;

	/// (͡ ͡° ͜ つ ͡͡°)
	fn schnose(&self) -> &'static str;

	/// Get the default footer used for embeds
	fn footer<'f>(&'_ self, footer: &'f mut CreateEmbedFooter) -> &'f mut CreateEmbedFooter;

	fn http(&self) -> &Http;
	fn global_maps(&self) -> &'static Vec<GlobalMap>;
	fn gokz_client(&self) -> &gokz_rs::Client;

	/// Fetches a single user from the database
	async fn get_user_by_id(
		&self,
		user_id: impl Into<u64> + Send,
	) -> Result<Option<database::User>>;
}

#[async_trait]
impl State for Context<'_> {
	fn state(&self) -> &GlobalState {
		self.framework().user_data
	}

	fn owner(&self) -> UserId {
		self.framework().user_data.owner_id
	}

	fn logs_channel(&self) -> ChannelId {
		self.framework().user_data.logs_channel
	}

	fn logs_table(&self) -> &str {
		self.framework()
			.user_data
			.logs_table
			.as_str()
	}

	fn users_table(&self) -> &str {
		self.framework()
			.user_data
			.users_table
			.as_str()
	}

	fn db(&self) -> &Pool<Postgres> {
		&self.framework().user_data.database_pool
	}

	fn color(&self) -> (u8, u8, u8) {
		self.framework().user_data.color
	}

	fn icon(&self) -> &String {
		&self.framework().user_data.icon
	}

	fn schnose(&self) -> &'static str {
		"(͡ ͡° ͜ つ ͡͡°)"
	}

	fn footer<'f>(&'_ self, footer: &'f mut CreateEmbedFooter) -> &'f mut CreateEmbedFooter {
		footer
			.icon_url(self.icon())
			.text(self.schnose())
	}

	fn global_maps(&self) -> &'static Vec<GlobalMap> {
		self.framework().user_data.global_maps
	}

	fn http(&self) -> &Http {
		&self.serenity_context().http
	}

	fn gokz_client(&self) -> &gokz_rs::Client {
		&self.framework().user_data.gokz_client
	}

	async fn get_user_by_id(
		&self,
		user_id: impl Into<u64> + Send,
	) -> Result<Option<database::User>> {
		let mut query = QueryBuilder::new("SELECT * FROM ");

		query
			.push(self.users_table())
			.push(" WHERE discord_id = ")
			.push_bind(user_id.into() as i64);

		let Some(user) = query
			.build_query_as::<database::UserRow>()
			.fetch_optional(self.db())
			.await? else {
			return Ok(None);
		};

		let user = database::User::from_row(user, self).await?;

		Ok(Some(user))
	}
}
