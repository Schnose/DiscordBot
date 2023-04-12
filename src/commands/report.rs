use {
	crate::{
		config,
		error::{Error, Result},
		state::{Context, State, StateContainer},
	},
	chrono::Utc,
	poise::{serenity_prelude::ChannelId, ApplicationContext, Modal},
	std::time::Duration,
};

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
#[tracing::instrument(skip(ctx), fields(user = ctx.author().tag()))]
#[poise::command(slash_command, on_error = "Error::handle")]
pub async fn report(ctx: ApplicationContext<'_, State, Error>) -> Result<()> {
	let Some(modal) = poise::execute_modal(ctx, Some(Report::default()), Some(Duration::from_secs(300))).await? else {
		// User didn't submit modal in time.
		return Ok(());
	};

	let ctx = Context::from(ctx);
	let channel = ChannelId(match &ctx.config().environment {
		config::Environment::Development { report_channel_id, .. }
		| config::Environment::Production { report_channel_id, .. } => *report_channel_id,
	});

	channel
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
								ctx.author_id(),
								Utc::now().format("%d/%m/%Y - %H:%M:%S")
							))
							.icon_url(ctx.icon_url())
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
