use {
	crate::{
		config,
		error::{Error, Result},
		state::{Context, StateContainer},
	},
	gokz_rs::SteamID,
	sqlx::QueryBuilder,
};

/// Save your SteamID in the bot's database.
///
/// This command will save your `SteamID` in its database for later use. Since many commands have \
/// a `player` parameter you probably don't want to specify that over and over again. Instead you \
/// can use this command and the bot will remember your choice in the future.
#[tracing::instrument(skip(ctx), fields(user = ctx.author().tag()))]
#[poise::command(slash_command, ephemeral, on_error = "Error::handle")]
pub async fn setsteam(
	ctx: Context<'_>,

	#[description = "Your SteamID, e.g. `STEAM_1:1:161178172` or `76561198282622073`"]
	steam_id: String,
) -> Result<()> {
	ctx.defer().await?;

	let steam_id = SteamID::new(&steam_id)?;

	let (name, id) = (&ctx.author().name, ctx.author_id());

	let table_name = match &ctx.config().environment {
		config::Environment::Development { users_table, .. }
		| config::Environment::Production { users_table, .. } => users_table,
	};

	let updated = match ctx.fetch_user_by_id(id).await {
		// User already has a database entry -> modify the current one
		Some(user) => {
			if user.steam_id.as_ref() == Some(&steam_id) {
				// The user already has a SteamID set and tried to set it to the same value.
				// :tf:
				ctx.say("<:tf:999383331647012935>")
					.await?;
				return Ok(());
			}

			let mut query = QueryBuilder::new(format!(
				r#"
				UPDATE {table_name}
				SET steam_id =
				"#
			));

			query
				.push_bind(steam_id.as_id32() as i32)
				.push(" WHERE discord_id = ")
				.push_bind(id as i64);

			query.build().execute(ctx.db()).await?;

			true
		}

		// The user does not yet have an entry -> create one
		None => {
			let mut query = QueryBuilder::new(format!(
				r#"
				INSERT INTO {table_name}
				    (name, discord_id, steam_id)
				"#
			));

			query.push_values([(name, id, steam_id)], |mut query, (name, id, steam_id)| {
				query
					.push_bind(name)
					.push_bind(id as i64)
					.push_bind(steam_id.as_id32() as i32);
			});

			query.build().execute(ctx.db()).await?;

			// We did not update an entry, but created one
			false
		}
	};

	ctx.say(match updated {
		true => format!("Successfully updated SteamID for <@{id}>! New SteamID: `{steam_id}`"),
		false => format!("Successfully set SteamID `{steam_id}` for <@{id}>!"),
	})
	.await?;

	Ok(())
}
