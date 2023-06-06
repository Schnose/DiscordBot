use crate::{Context, Error, Result};

#[poise::command(slash_command, on_error = "Error::global_handler")]
pub async fn ping(ctx: Context<'_>) -> Result<()> {
	ctx.send(|reply| reply.content("Pong!"))
		.await?;

	Ok(())
}
