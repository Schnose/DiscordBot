use crate::{Context, Error, GlobalState, Result, State};
use poise::{ApplicationContext, Modal};
use sqlx::types::chrono::Utc;
use std::time::Duration;

#[derive(Debug, Default, Modal)]
#[name = "Report Issue / Suggest change"]
struct Report {
	#[name = "Title"]
	#[placeholder = "<title>"]
	title: String,

	#[name = "Description"]
	#[placeholder = "Describe your issue here. Please provide Screenshots if you can."]
	#[paragraph]
	description: String,
}

/// Report issues/bugs with the bot or suggest changes.
///
/// This command will open a pop-up where you can submit bug reports / suggestions for the bot (in \
/// case you don't like GitHub issues). The information you put in there will be sent to a channel \
/// that can be specified in the bot's config file. If you use my instance of the bot, that \
/// channel is a private channel on my Discord server that only I and a few admins have access to.
#[poise::command(slash_command, on_error = "Error::global_handler")]
pub async fn report(ctx: ApplicationContext<'_, GlobalState, Error>) -> Result<()> {
	let Some(modal) = poise::execute_modal(ctx, Some(Report::default()), Some(Duration::from_secs(300))).await? else {
		// User didn't submit modal in time.
		return Ok(());
	};

	let ctx = Context::from(ctx);

	ctx.reports_channel()
		.send_message(&ctx.serenity_context().http, |message| {
			message.embed(|embed| {
				embed
					.color(ctx.color())
					.title(modal.title)
					.description(modal.description)
					.thumbnail(
						ctx.author()
							.avatar_url()
							.unwrap_or_else(|| ctx.author().default_avatar_url()),
					)
					.footer(|footer| {
						footer
							.text(format!(
								"User: {} ({}) | {}",
								ctx.author().tag(),
								ctx.author().id,
								Utc::now().format("%d/%m/%Y - %H:%M:%S")
							))
							.icon_url(ctx.icon())
					})
			})
		})
		.await?;

	ctx.send(|reply| {
		reply
			.ephemeral(true)
			.content("Thank you for the submission!")
	})
	.await?;

	Ok(())
}
