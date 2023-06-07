use super::params;
use crate::{Context, Error, Result, State};
use gokz_rs::{global_api, kzgo_api};
use poise::serenity_prelude::CreateEmbed;
use std::ops::Range;

/// Top 100 world record holders.
///
/// This command will fetch the top 100 world record holders. You may specify the following \
/// options:
///
/// - `mode`: `KZTimer` / `SimpleKZ` / `Vanilla`
///   - If you don't specify this, the bot will search the database for your UserID. If it can't \
///     find one, or you don't have a mode preference set, the command will fail. To save a mode \
///     preference in the database, see `/mode`.
/// - `runtype`: `TP` / `PRO`
///   - If you don't specify this, the bot will default to `PRO`.
#[poise::command(slash_command, on_error = "Error::global_handler")]
pub async fn top(
	ctx: Context<'_>,

	#[description = "KZT/SKZ/VNL"] mode: Option<params::Mode>,

	#[description = "TP/PRO"] runtype: Option<params::Runtype>,
) -> Result<()> {
	ctx.defer().await?;

	_top(ctx, mode, runtype, 0..1).await?;

	Ok(())
}

/// Top 100 bonus world record holders.
///
/// This command will fetch the top 100 world record holders for bonuses. You may specify the \
/// following options:
///
/// - `mode`: `KZTimer` / `SimpleKZ` / `Vanilla`
///   - If you don't specify this, the bot will search the database for your UserID. If it can't \
///     find one, or you don't have a mode preference set, the command will fail. To save a mode \
///     preference in the database, see `/mode`.
/// - `runtype`: `TP` / `PRO`
///   - If you don't specify this, the bot will default to `PRO`.
#[poise::command(slash_command, on_error = "Error::global_handler")]
pub async fn btop(
	ctx: Context<'_>,

	#[description = "KZT/SKZ/VNL"] mode: Option<params::Mode>,

	#[description = "TP/PRO"] runtype: Option<params::Runtype>,
) -> Result<()> {
	ctx.defer().await?;

	_top(ctx, mode, runtype, 1..101).await?;

	Ok(())
}

async fn _top(
	ctx: Context<'_>,
	mode: Option<params::Mode>,
	runtype: Option<params::Runtype>,
	range: Range<u8>,
) -> Result<()> {
	let mode = params::Mode::parse_param(mode, &ctx).await?;
	let runtype = runtype.unwrap_or_default();
	let top = global_api::get_wr_top(mode, runtype.into(), range, ctx.gokz_client())
		.await?
		.into_iter()
		.take(100)
		.collect::<Vec<_>>();

	let wr_avatar = kzgo_api::get_avatar(top[0].steam_id, ctx.gokz_client())
		.await
		.map(|user| user.avatar_url)
		.unwrap_or_default();

	assert!(
		!top.is_empty(),
		"Leaderboard can't be empty, since there's more than 0 world records."
	);

	let mut embeds = Vec::new();
	let mut temp = CreateEmbed::default();

	let players_per_page = 12;
	let max_pages = (top.len() as f64 / players_per_page as f64).ceil() as u8;
	let mut place = 1;

	for (page_idx, players) in top.chunks(players_per_page).enumerate() {
		let title = format!("[{} {:?}] Top 100 WR holders", mode.short(), runtype);
		let url = format!("https://kzgo.eu/leaderboards?{}=", mode.short().to_lowercase());
		let thumbnail = &wr_avatar;

		temp.color(ctx.color())
			.title(title)
			.url(url)
			.thumbnail(thumbnail)
			.footer(|footer| {
				footer
					.icon_url(ctx.icon())
					.text(format!(
						"{schnose} | Page {current} / {max}",
						schnose = ctx.schnose(),
						current = page_idx + 1,
						max = max_pages
					))
			});

		for player in players {
			let title = format!("{player} [#{place}]", player = player.player_name);
			temp.field(title, player.count, true);
			place += 1;
		}

		embeds.push(temp);
		temp = Default::default();
	}

	match embeds.len() {
		0 => unreachable!(),

		1 => {
			ctx.send(|reply| {
				reply.embed(|embed| {
					*embed = embeds.remove(0);
					embed
				})
			})
			.await?;
		}

		_ => super::pagination::paginate(&ctx, &embeds).await?,
	};

	Ok(())
}
