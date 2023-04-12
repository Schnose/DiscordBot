use {
	super::custom_params::DBModeChoice,
	crate::{
		config,
		error::{Error, Result},
		state::{Context, StateContainer},
	},
	gokz_rs::Mode,
	sqlx::QueryBuilder,
};

/// Set your mode preference.
///
/// This command will save your mode preference in its database for later use. Since many commands \
/// have a `mode` parameter you probably don't want to specify that over and over again. Instead \
/// you can use this command and the bot will remember your choice in the future. You can also \
/// clear your preference if you want to.
#[tracing::instrument(skip(ctx), fields(user = ctx.author().tag()))]
#[poise::command(slash_command, ephemeral, on_error = "Error::handle")]
pub async fn mode(
	ctx: Context<'_>,

	#[description = "None/KZT/SKZ/VNL"]
	#[rename = "mode"]
	mode_choice: Option<DBModeChoice>,
) -> Result<()> {
	ctx.defer().await?;

	let mode: Option<Mode> = mode_choice.and_then(|choice| choice.into());

	let (name, id) = (&ctx.author().name, ctx.author_id());

	let table_name = match &ctx.config().environment {
		config::Environment::Development { users_table, .. }
		| config::Environment::Production { users_table, .. } => users_table,
	};

	let updated = match ctx.fetch_user_by_id(id).await {
		// User already has a database entry -> modify the current one
		Some(user) => {
			if user.mode.as_ref() == mode.as_ref() {
				// The user already has a mode set and tried to set it to the same value.
				// :tf:
				ctx.say("<:tf:999383331647012935>")
					.await?;
				return Ok(());
			}

			let mut query = QueryBuilder::new(format!(
				r#"
				UPDATE {table_name}
				SET mode =
				"#
			));

			query
				.push_bind(mode.map(|mode| mode as u8 as i16))
				.push(" WHERE discord_id = ")
				.push_bind(id as i64);

			query.build().execute(ctx.db()).await?;

			true
		}

		// The user does not yet have an entry -> create one
		None => {
			let Some(mode) = mode else {
				// The user does not yet have an entry but told us to clear their current entry.
				// :tf:
				ctx.say("<:tf:999383331647012935>").await?;
				return Ok(());
			};

			let mut query = QueryBuilder::new(format!(
				r#"
				INSERT INTO {table_name}
				    (name, discord_id, mode)
				"#
			));

			query.push_values([(name, id, mode)], |mut query, (name, id, mode)| {
				query
					.push_bind(name)
					.push_bind(id as i64)
					.push_bind(mode as u8 as i16);
			});

			query.build().execute(ctx.db()).await?;

			// We did not update an entry, but created one
			false
		}
	};

	ctx.say(match mode {
		None => format!("Successfully cleared Mode for <@{id}>!"),
		Some(mode) if updated => {
			format!("Successfully updated Mode for <@{id}>! New Mode: `{mode}`")
		}
		Some(mode) => format!("Successfully set Mode `{mode}` for <@{id}>!"),
	})
	.await?;

	Ok(())
}
