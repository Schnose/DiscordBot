use crate::{Context, Error, Result};

static INVITE_LINK: &str = "https://discord.com/oauth2/authorize?client_id=940308056451973120&permissions=327744&scope=bot%20applications.commands";

/// Invite schnose to your own server!
#[poise::command(slash_command, on_error = "Error::global_handler", ephemeral)]
pub async fn invite(ctx: Context<'_>) -> Result<()> {
	ctx.say(format!("[click me?]({INVITE_LINK}) 😳 👉👈"))
		.await?;

	Ok(())
}
