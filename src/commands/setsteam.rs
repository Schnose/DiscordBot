use crate::{Context, Error, Result};
use gokz_rs::SteamID;
use schnose_discord_bot::State;
use sqlx::QueryBuilder;

/// Save your SteamID in the bot's database.
///
/// This command will save your `SteamID` in the bot's database so it can be use for other \
/// commands such as `/pb`. By not specifiying the `steam_id` parameter you can delete your \
/// current entry from the database.
#[poise::command(slash_command, on_error = "Error::global_handler")]
pub async fn setsteam(
	ctx: Context<'_>,
	#[description = "Your SteamID, e.g. `STEAM_1:1:161178172` or `76561198282622073`"]
	steam_id: Option<SteamID>,
) -> Result<()> {
	ctx.defer().await?;

	let mut query = QueryBuilder::new("");
	let user_id = *ctx.author().id.as_u64();
	let steam_id_db = steam_id.map(|steam_id| steam_id.as_id32() as i32);

	let had_steam_id = match ctx.get_user_by_id(user_id).await? {
		Some(ref user) => {
			// User has an entry => modify that

			query
				.push("UPDATE ")
				.push(ctx.users_table())
				.push(" SET steam_id = ")
				.push_bind(steam_id_db);

			query
				.push(" WHERE discord_id = ")
				.push_bind(user_id as i64);

			user.steam_id.is_some()
		}

		None => {
			// User does not have an entry => create a new one

			let name = &ctx.author().name;

			query
				.push("INSERT INTO ")
				.push(ctx.users_table())
				.push(" (name, discord_id, steam_id) ")
				.push_values([(name, user_id)], |mut query, (name, discord_id)| {
					query
						.push_bind(name)
						.push_bind(discord_id as i64)
						.push_bind(steam_id_db);
				});

			false
		}
	};

	query.build().execute(ctx.db()).await?;

	let steam_id = steam_id
		.map(|steam_id| steam_id.to_string())
		.unwrap_or_else(|| String::from("NULL"));

	let message = if steam_id_db.is_none() {
		format!("Successfully cleared SteamID for <@{user_id}>!")
	} else if had_steam_id {
		format!("Successfully updated SteamID for <@{user_id}>! (New SteamID: `{steam_id}`)")
	} else {
		format!("Successfully set SteamID `{steam_id}` for <@{user_id}>!")
	};

	ctx.say(message).await?;

	Ok(())
}
