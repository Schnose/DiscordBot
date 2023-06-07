use crate::{Context, Error, Result};

/// Pong!
#[poise::command(slash_command, on_error = "Error::global_handler")]
pub async fn ping(ctx: Context<'_>) -> Result<()> {
	ctx.say("Pong!").await?;

	Ok(())
}
