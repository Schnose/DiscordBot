use super::params;
use crate::{Context, Error, Result};
use schnose_discord_bot::State;
use sqlx::QueryBuilder;

/// Save your favorite game mode in the bot's database.
///
/// This command will save your preferred KZ mode in the bot's database so it can be used for \
/// other commands such as `/pb`. By not specifiying the `mode` parameter you can delete your \
/// current entry from the database.
#[poise::command(slash_command, on_error = "Error::global_handler")]
pub async fn mode(
	ctx: Context<'_>,

	#[description = "Your favorite KZ mode"] mode: Option<params::Mode>,
) -> Result<()> {
	ctx.defer().await?;

	let mut query = QueryBuilder::new("");
	let user_id = *ctx.author().id.as_u64();
	let mode_db = mode.map(|mode| mode as u8 as i16);

	let had_mode = match ctx.get_user_by_id(user_id).await? {
		Some(ref user) => {
			// User has an entry => modify that

			query
				.push("UPDATE ")
				.push(ctx.users_table())
				.push(" SET mode = ")
				.push_bind(mode_db);

			query
				.push(" WHERE discord_id = ")
				.push_bind(user_id as i64);

			user.mode.is_some()
		}

		None => {
			// User does not have an entry => create a new one

			let name = &ctx.author().name;

			query
				.push("INSERT INTO ")
				.push(ctx.users_table())
				.push(" (name, discord_id, mode) ")
				.push_values([(name, user_id)], |mut query, (name, discord_id)| {
					query
						.push_bind(name)
						.push_bind(discord_id as i64)
						.push_bind(mode_db);
				});

			false
		}
	};

	query.build().execute(ctx.db()).await?;

	let mode = mode
		.map(|mode| mode.to_string())
		.unwrap_or_else(|| String::from("NULL"));

	let message = if mode_db.is_none() {
		format!("Successfully cleared Mode for <@{user_id}>!")
	} else if had_mode {
		format!("Successfully updated Mode for <@{user_id}>! (New Mode: `{mode}`)")
	} else {
		format!("Successfully set Mode `{mode}` for <@{user_id}>!")
	};

	ctx.say(message).await?;

	Ok(())
}
