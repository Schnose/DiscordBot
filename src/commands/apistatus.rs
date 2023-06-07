use crate::{Context, Error, Result};
use gokz_rs::global_api::{self, HealthReport};
use schnose_discord_bot::State;

/// GlobalAPI health report.
///
/// Both this bot and GOKZ rely on the \
/// [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2) to function \
/// properly. Sometimes it has downtimes though, and the bot commands might not work. This command \
/// will give you some information about the \
/// [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2)'s current \
/// status. It uses [this website](https://health.global-api.com/endpoints/_globalapi) for \
/// fetching that information and displays different messages depending on the current stats.
#[poise::command(slash_command, on_error = "Error::global_handler")]
pub async fn apistatus(ctx: Context<'_>) -> Result<()> {
	ctx.defer().await?;

	let HealthReport { successful_responses, fast_responses } =
		global_api::checkhealth(ctx.gokz_client()).await?;

	let average = (successful_responses as f64 + fast_responses as f64) / 2_f64;
	let success = (average * 10_f64) as u8;
	let (status, color) = match success {
		90.. => ("Healthy", (116, 227, 161)),
		67.. => ("<:schnosesus:947467755727241287>", (249, 226, 175)),
		33.. => ("everything is on fire", (250, 179, 135)),
		_ => ("zer0.k wanted to be funny and pulled the usb stick again", (243, 139, 168)),
	};

	ctx.send(|reply| {
		reply.embed(|embed| {
			embed.color(color)
			.title(status)
				.url("https://health.global-api.com/endpoints/_globalapi")
				.thumbnail("https://dka575ofm4ao0.cloudfront.net/pages-transactional_logos/retina/74372/kz-icon.png")
				.field("Successful Healthchecks", format!("{successful_responses} / {}", 10), true)
				.field("Fast Responses", format!("{fast_responses} / {}", 10), true)
				.footer(|footer| ctx.footer(footer))
		})
	}).await?;

	Ok(())
}
