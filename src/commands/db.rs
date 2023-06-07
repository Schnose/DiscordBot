use super::params;
use crate::{database, Context, Error, Result};
use poise::serenity_prelude::Member;
use schnose_discord_bot::State;

/// Check a user's database entries.
///
/// This command will show you all the information the bot has saved about a user. You may \
/// toggle a `public` flag so the bot sends a message everyone can see.
#[poise::command(slash_command, on_error = "Error::global_handler")]
pub async fn db(
	ctx: Context<'_>,

	#[description = "The user you want to retrieve information about."] user: Option<Member>,
	#[description = "Make the message visible for other people."] public: Option<params::Bool>,
) -> Result<()> {
	if matches!(public.as_deref(), Some(true)) {
		ctx.defer().await?;
	} else {
		ctx.defer_ephemeral().await?;
	}

	let (user_id, user_name) = user
		.map(|member| {
			let user = member.user;
			(user.id, user.name)
		})
		.unwrap_or_else(|| {
			let user = ctx.author();
			(user.id, user.name.clone())
		});

	let Some(database::User {
		discord_id,
		name,
		steam_id,
		mode
	}) = ctx.get_user_by_id(user_id).await? else {
		ctx.say(format!("{user_name} has no database entries."))
		.await?;

		return Ok(());
	};

	let steam_id = steam_id
		.map(|steam_id| steam_id.to_string())
		.unwrap_or_else(|| String::from("None"));

	let mode = mode
		.map(|mode| mode.short())
		.unwrap_or_else(|| String::from("None"));

	let description = format!(
		r#"- `discord_id`: {discord_id}
- `name`: {name}
- `steam_id`: {steam_id}
- `mode`: {mode}"#
	);

	ctx.send(|reply| {
		reply.embed(|embed| {
			embed
				.color(ctx.color())
				.title(format!("{name}'s database entries"))
				.description(description)
				.footer(|footer| ctx.footer(footer))
		})
	})
	.await?;

	Ok(())
}
