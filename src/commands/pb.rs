use super::{autocomplete, params};
use crate::{Context, Error, Result, State};
use gokz_rs::{global_api, Mode};

/// A player's personal best on a map.
///
/// This command will fetch a player's personal best on a particular map. If there is a global \
/// replay available for any of your runs, the bot will attach some links for watching it online \
/// with [GC's replay viewer](https://github.com/GameChaos/GlobalReplays) as well as downloading \
/// the file. You are required to specify a `map` and may also specify the following options:
///
/// - `mode`: `KZTimer` / `SimpleKZ` / `Vanilla`
///   - If you don't specify this, the bot will search the database for your UserID. If it can't \
///     find one, or you don't have a mode preference set, the command will fail. To save a mode \
///     preference in the database, see `/mode`.
/// - `player`: this can be any string. The bot will try its best to interpret it as something \
///   useful. If you want to help it with that, specify one of the following:
///   - a `SteamID`, e.g. `STEAM_1:1:161178172`, `U:1:322356345` or `76561198282622073`
///   - a `Mention`, e.g. `@MyBestFriend`
///   - a player's name, e.g. `AlphaKeks`
///   - If you don't specify this, the bot will search the database for your UserID. If it can't \
///     find one, or you don't have a SteamID set, the command will fail. To save a mode \
///     preference in the database, see `/setsteam`.
#[poise::command(slash_command, on_error = "Error::global_handler")]
pub async fn pb(
	ctx: Context<'_>,

	#[description = "Choose a map"]
	#[autocomplete = "autocomplete::map_name"]
	map: autocomplete::GlobalMap,

	#[description = "KZT/SKZ/VNL"] mode: Option<params::Mode>,

	#[description = "The player you want to check."] player: Option<params::Target>,
) -> Result<()> {
	ctx.defer().await?;

	let course = 0;
	_pb(ctx, map, mode, player, course).await?;

	Ok(())
}

/// A player's personal best on a bonus course.
///
/// This command will fetch a player's personal best on a particular bonus. You are required to \
/// specify a `map` and may also specify the following options:
///
/// - `mode`: `KZTimer` / `SimpleKZ` / `Vanilla`
///   - If you don't specify this, the bot will search the database for your UserID. If it can't \
///     find one, or you don't have a mode preference set, the command will fail. To save a mode \
///     preference in the database, see `/mode`.
/// - `player`: this can be any string. The bot will try its best to interpret it as something \
///   useful. If you want to help it with that, specify one of the following:
///   - a `SteamID`, e.g. `STEAM_1:1:161178172`, `U:1:322356345` or `76561198282622073`
///   - a `Mention`, e.g. `@MyBestFriend`
///   - a player's name, e.g. `AlphaKeks`
///   - If you don't specify this, the bot will search the database for your UserID. If it can't \
///     find one, or you don't have a SteamID set, the command will fail. To save a mode \
///     preference in the database, see `/setsteam`.
/// - `course`: this can be any integer between 1-255.
///   - If you either don't specify this, or put in `0`, the bot will default to `1`.
#[poise::command(slash_command, on_error = "Error::global_handler")]
pub async fn bpb(
	ctx: Context<'_>,

	#[description = "Choose a map"]
	#[autocomplete = "autocomplete::map_name"]
	map: autocomplete::GlobalMap,

	#[description = "KZT/SKZ/VNL"] mode: Option<params::Mode>,

	#[description = "The player you want to check."] player: Option<params::Target>,

	#[description = "Which bouns?"]
	#[min = 1]
	#[max = 100]
	course: Option<u8>,
) -> Result<()> {
	ctx.defer().await?;

	let course = course.unwrap_or(1);
	_pb(ctx, map, mode, player, course).await?;

	Ok(())
}

async fn _pb(
	ctx: Context<'_>,
	map: autocomplete::GlobalMap,
	mode: Option<params::Mode>,
	player: Option<params::Target>,
	course: u8,
) -> Result<()> {
	let mode = params::Mode::parse_param(mode, &ctx)
		.await
		.unwrap_or(Mode::KZTimer);

	let player_identifier = player
		.unwrap_or(params::Target::None(ctx.author().id))
		.into_player(&ctx)
		.await;

	let tp_wr = global_api::get_pb(
		player_identifier.clone(),
		map.id.into(),
		mode,
		true,
		course,
		ctx.gokz_client(),
	)
	.await;

	let pro_wr = global_api::get_pb(
		player_identifier.clone(),
		map.id.into(),
		mode,
		false,
		course,
		ctx.gokz_client(),
	)
	.await;

	if tp_wr.is_err() && pro_wr.is_err() {
		return Err(Error::NoRecords);
	}

	let ((tp_time, tp_links), (pro_time, pro_links)) =
		crate::global_api::parse_records(&tp_wr, &pro_wr, Some(ctx.gokz_client())).await;

	let mut player_name = &ctx.author().name;

	if let Ok(ref rec) = tp_wr {
		player_name = &rec.player_name;
	} else if let Ok(ref rec) = pro_wr {
		player_name = &rec.player_name;
	}

	let mut title = format!("[PB] {} on {}", player_name, map.name);

	if course > 0 {
		use std::fmt::Write;
		write!(&mut title, " B{}", course)?;
	};

	let url = format!("{}?{}=", map.kzgo_link(), mode.short().to_lowercase());
	let thumbnail = map.thumbnail();

	let player_links = crate::global_api::get_player_info(tp_wr, pro_wr).unwrap_or_default();

	let replay_links =
		crate::global_api::format_replay_links(tp_links, pro_links).unwrap_or_default();

	let description = format!("{player_links}\n\n{replay_links}");

	ctx.send(|reply| {
		reply.embed(|embed| {
			embed
				.color(ctx.color())
				.title(title)
				.url(url)
				.thumbnail(thumbnail)
				.description(description)
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
