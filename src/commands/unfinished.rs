use super::params;
use crate::{Context, Error, Result, State};
use gokz_rs::{global_api, kzgo_api, schnose_api, Mode};
use poise::serenity_prelude::CreateEmbed;

/// Check which maps you still need to finish.
///
/// This command will fetch all maps that you haven't finished yet in a particular mode. You may \
/// specify the following parameters:
///
/// - `mode`: `KZTimer` / `SimpleKZ` / `Vanilla`
///   - If you don't specify this, the bot will search the database for your UserID. If it can't \
///     find one, or you don't have a mode preference set, the command will fail. To save a mode \
///     preference in the database, see `/mode`.
/// - `runtype`: `TP` / `PRO`
///   - If you don't specify this, the bot will default to `PRO`.
/// - `tier`: If you don't specify this, the bot will fetch maps for all tiers.
/// - `player`: this can be any string. The bot will try its best to interpret it as something \
///   useful. If you want to help it with that, specify one of the following:
///   - a `SteamID`, e.g. `STEAM_1:1:161178172`, `U:1:322356345` or `76561198282622073`
///   - a `Mention`, e.g. `@MyBestFriend`
///   - a player's name, e.g. `AlphaKeks`
///   - If you don't specify this, the bot will search the database for your UserID. If it can't \
///     find one, or you don't have a SteamID set, the command will fail. To save a mode \
///     preference in the database, see `/setsteam`.
#[poise::command(slash_command, on_error = "Error::global_handler")]
pub async fn unfinished(
	ctx: Context<'_>,

	#[description = "KZT/SKZ/VNL"] mode: Option<params::Mode>,

	#[description = "TP/PRO"] runtype: Option<params::Runtype>,

	#[description = "Fitler by difficulty"] tier: Option<params::Tier>,

	#[description = "The player you want to check."] player: Option<params::Target>,
) -> Result<()> {
	ctx.defer().await?;

	let mode = params::Mode::parse_param(mode, &ctx)
		.await
		.unwrap_or(Mode::KZTimer);

	let runtype = runtype.unwrap_or_default();

	let player_identifier = player
		.unwrap_or(params::Target::None(ctx.author().id))
		.into_player(&ctx)
		.await;

	let player = schnose_api::get_player(player_identifier.clone(), ctx.gokz_client()).await?;
	let unfinished = global_api::get_unfinished(
		player_identifier,
		mode,
		runtype.into(),
		tier.map(From::from),
		ctx.gokz_client(),
	)
	.await?
	.map(|maps| {
		maps.into_iter()
			.enumerate()
			.map(|(idx, map)| {
				if tier.is_some() {
					map.name
				} else {
					format!(
						"{i}. [{map}](https://kzgo.eu/maps/{map}?{mode}=) (T{tier})",
						i = idx + 1,
						map = map.name,
						mode = mode.short().to_lowercase(),
						tier = map.difficulty as u8,
					)
				}
			})
			.collect::<Vec<_>>()
	});

	let avatar = kzgo_api::get_avatar(player.steam_id, ctx.gokz_client())
		.await
		.map(|user| user.avatar_url)
		.unwrap_or_default();

	let title = format!(
		"{mode} {runtype:?} {tier}",
		mode = mode.short(),
		tier = tier.map_or_else(String::new, |tier| format!("[T{}]", tier as u8)),
	);

	let url = format!(
		"https://kzgo.eu/players/{steam_id}?{mode}=",
		steam_id = player.steam_id,
		mode = mode.short().to_lowercase(),
	);

	let mut template = CreateEmbed::default();
	template
		.color(ctx.color())
		.title(title)
		.url(url)
		.thumbnail(avatar)
		.description("Congrats! You have no maps left to finish! 🥳")
		.footer(|footer| {
			footer
				.icon_url(ctx.icon())
				.text(format!("Player: {}", player.name))
		});

	match unfinished {
		None => {
			ctx.send(|reply| {
				reply.embed(|embed| {
					*embed = template;
					embed
				})
			})
			.await?;
		}

		Some(maps) if maps.len() <= 10 => {
			let description = maps.join("\n");

			ctx.send(|reply| {
				reply.embed(|embed| {
					template.description(description);
					*embed = template;
					embed
				})
			})
			.await?;
		}

		Some(maps) => {
			let mut embeds = Vec::new();
			let n_maps = maps.len();
			let maps_per_page = 12;
			let max_pages = (maps.len() as f64 / maps_per_page as f64).ceil() as u8;

			for (page_idx, maps) in maps.chunks(maps_per_page).enumerate() {
				let mut temp = template.clone();
				let title = format!(
					"{n_maps} maps - {mode} {runtype:?} {tier}",
					mode = mode.short(),
					tier = tier.map_or_else(String::new, |tier| format!("[T{}]", tier as u8)),
				);

				let description = maps.join("\n");

				temp.title(title)
					.description(description)
					.footer(|footer| {
						footer
							.icon_url(ctx.icon())
							.text(format!(
								"Player: {name} | Page {current} / {max}",
								name = player.name,
								current = page_idx + 1,
								max = max_pages,
							))
					});

				embeds.push(temp);
			}

			super::pagination::paginate(&ctx, &embeds).await?;
		}
	};

	Ok(())
}
