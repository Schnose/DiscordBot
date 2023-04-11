use crate::{error::Result, state::Context};

/// Pong!
#[tracing::instrument(skip(ctx), fields(user = ctx.author().tag()))]
#[poise::command(slash_command, ephemeral)]
pub async fn ping(ctx: Context<'_>) -> Result<()> {
	ctx.say("Pong!").await?;

	Ok(())
}
