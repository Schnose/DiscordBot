use crate::{error::Result, state::Context};

/// Invite schnose to your own server!
#[tracing::instrument(skip(ctx), fields(user = ctx.author().tag()))]
#[poise::command(slash_command, ephemeral)]
pub async fn invite(ctx: Context<'_>) -> Result<()> {
	ctx.say("[click me? 😳](<https://discord.com/oauth2/authorize?client_id=940308056451973120&permissions=327744&scope=bot%20applications.commands>)").await?;

	Ok(())
}
