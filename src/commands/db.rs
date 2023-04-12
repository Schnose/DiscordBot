use {
	super::custom_params::BoolChoice,
	crate::{
		database,
		error::{Error, Result},
		state::{Context, StateContainer},
		target::Target,
	},
};

/// Check your database entries.
///
/// This command will show you all the information that the bot has saved about your account in \
/// its database. You may specify a `public` option that determines whether other people will be \
/// able to see the bot's response or not.
#[tracing::instrument(skip(ctx), fields(user = ctx.author().tag()))]
#[poise::command(slash_command, ephemeral, on_error = "Error::handle")]
pub async fn db(
	ctx: Context<'_>,

	#[description = "Send the message so that everyone can see it."]
	#[rename = "public"]
	show_message: Option<BoolChoice>,
) -> Result<()> {
	if matches!(show_message, Some(BoolChoice::Yes)) {
		ctx.defer().await?;
	} else {
		ctx.defer_ephemeral().await?;
	}

	let user_id = *ctx.author().id.as_u64();

	let database::User { name, discord_id, steam_id, mode } =
		ctx.fetch_user_by_id(user_id)
			.await
			.ok_or(Error::UserNotInDatabase { user: Target::None { user_id } })?;

	let steam_id = steam_id
		.map(|steam_id| steam_id.to_string())
		.unwrap_or_else(|| String::from("NULL"));

	let mode = mode
		.map(|mode| mode.short())
		.unwrap_or_else(|| String::from("NULL"));

	let description = format!(
		r#"
> `name`: `{name}`
> `discord_id`: `{discord_id}`
> `steam_id`: `{steam_id}`
> `mode`: `{mode}`
		"#
	);

	ctx.send(|reply| {
		reply.embed(|embed| {
			embed
				.color(ctx.color())
				.title(format!("{name}'s database entry"))
				.description(description)
				.footer(|footer| {
					footer
						.text(ctx.schnose())
						.icon_url(ctx.icon_url())
				})
		})
	})
	.await?;

	Ok(())
}
