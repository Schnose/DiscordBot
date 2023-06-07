use super::{autocomplete, params};
use crate::{Context, Error, Result, State};
use gokz_rs::global_api;

/// Get detailed information on a map.
///
/// This command will fetch a bunch of useful information about a particular map. The information \
/// is a combination of the \
/// [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2), \
/// [n4vyn's](https://github.com/n4vyn) [KZ:GO API](https://kzgo.eu/) and my own \
/// [SchnoseAPI](https://github.com/Schnose/SchnoseAPI). If anything seems incorrect, feel free \
/// to report it.
#[poise::command(slash_command, on_error = "Error::global_handler")]
pub async fn wr(
	ctx: Context<'_>,

	#[description = "Choose a map"]
	#[autocomplete = "autocomplete::map_name"]
	map: autocomplete::GlobalMap,

	#[description = "KZT/SKZ/VNL"] mode: Option<params::Mode>,
) -> Result<()> {
	ctx.defer().await?;

	let mode = params::Mode::parse_param(mode, &ctx).await?;
	let tp_wr = global_api::get_wr(map.id.into(), mode, true, 0, ctx.gokz_client()).await;
	let pro_wr = global_api::get_wr(map.id.into(), mode, false, 0, ctx.gokz_client()).await;

	let ((tp_time, tp_links), (pro_time, pro_links)) =
		crate::global_api::parse_records(tp_wr, pro_wr);

	let title = format!("[WR] {}", map.name);
	let url = format!("{}?{}=", map.kzgo_link(), mode.short().to_lowercase());
	let thumbnail = map.thumbnail();
	let replay_links =
		crate::global_api::format_replay_links(tp_links, pro_links).unwrap_or_default();

	ctx.send(|reply| {
		reply.embed(|embed| {
			embed
				.color(ctx.color())
				.title(title)
				.url(url)
				.thumbnail(thumbnail)
				.description(replay_links)
				.field("TP", tp_time, true)
				.field("PRO", pro_time, true)
				.footer(|footer| {
					footer
						.icon_url(ctx.icon())
						.text(format!(
							"{schnose} | Mode: {mode}",
							schnose = ctx.schnose(),
							mode = mode.short()
						))
				})
		})
	})
	.await?;

	Ok(())
}
