use {
	super::pagination,
	crate::{
		error::{Error, Result},
		state::{Context, StateContainer},
		target::Target,
	},
	gokz_rs::{global_api, schnose_api},
	poise::serenity_prelude::CreateEmbed,
	schnosebot::time,
};

/// Get a player's 10 most recent runs.
///
/// This command will fetch a player's most recent 10 runs (this includes non-pbs and bonus runs). \
/// If there is a global replay available for any of your runs, the bot will attach some links for \
/// watching it online with [GC's replay viewer](https://github.com/GameChaos/GlobalReplays) as \
/// well as downloading the file. You may specify a `player`, which can be any string. The bot \
/// will try its best to interpret it as something useful. If you want to help it with that, \
/// specify one of the following:
///   - a `SteamID`, e.g. `STEAM_1:1:161178172`, `U:1:322356345` or `76561198282622073`
///   - a `Mention`, e.g. `@MyBestFriend`
///   - a player's name, e.g. `AlphaKeks`
///   - If you don't specify this, the bot will search the database for your UserID. If it can't \
///     find one, or you don't have a SteamID set, the command will fail. To save a mode \
///     preference in the database, see `/setsteam`.
#[tracing::instrument(skip(ctx), fields(user = ctx.author().tag()))]
#[poise::command(slash_command, ephemeral, on_error = "Error::handle")]
pub async fn recent(
	ctx: Context<'_>,

	#[description = "The player you want to target."]
	#[rename = "player"]
	target: Option<String>,
) -> Result<()> {
	ctx.defer().await?;

	let target: Target = match target {
		None => ctx.author_id().into(),
		Some(target) => target.parse()?,
	};

	let player = target.into_player(&ctx).await;

	let recent_records = schnose_api::get_recent(player, 10, ctx.gokz_client()).await?;

	let mut embeds = Vec::new();

	// How many pages
	let max_pages = recent_records.len();

	for (page_idx, record) in recent_records.into_iter().enumerate() {
		let place = global_api::get_place(record.id, ctx.gokz_client())
			.await
			.map(|place| format!("[#{place}]"))
			.unwrap_or_default();

		let (map_name, map_tier, map_url, map_thumbnail) = ctx
			.get_map(record.map_name.clone())
			.map(|map| {
				(
					map.name.clone(),
					(map.tier as u8).to_string(),
					format!("{}?{}=", map.kzgo_link(), record.mode.short().to_lowercase()),
					map.thumbnail(),
				)
			})
			.unwrap_or_else(|| {
				(
					record.map_name.clone(),
					String::from("?"),
					String::new(),
					String::from("https://kzgo.eu/kz_default.png"),
				)
			});

		let teleports = match record.teleports {
			0 => String::new(),
			1 => String::from("(1 TP)"),
			n => format!("({n} TPs)"),
		};

		let discord_timestamp = format!("<t:{}:R>", record.created_on.timestamp());
		let player_links = format!(
			"Player: [KZ:GO](https://kzgo.eu/players/{}) | [Steam](https://steamcommunity.com/profiles/{})",
			record.player.steam_id,
			record.player.steam_id.as_id64()
		);

		let mut embed = CreateEmbed::default();
		embed
			.color(ctx.color())
			.title(format!(
				"{} on {}{} (T{})",
				&record.player.name,
				&map_name,
				if record.course.stage == 0 {
					String::new()
				} else {
					format!(" B{}", record.course.stage)
				},
				map_tier
			))
			.url(map_url)
			.thumbnail(map_thumbnail)
			.field(
				format!(
					"{} {}",
					record.mode.short(),
					if record.teleports > 0 { "TP" } else { "PRO" }
				),
				format!(
					"> {} {}{}\n> {}\n> {}",
					time::format(record.time),
					place,
					teleports,
					discord_timestamp,
					player_links
				),
				true,
			)
			.footer(|footer| {
				footer
					.text(format!(
						"{} | Record ID: {} | Page: {} / {}",
						ctx.schnose(),
						record.id,
						page_idx + 1,
						max_pages
					))
					.icon_url(ctx.icon_url())
			});

		embeds.push(embed);
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
		_ => pagination::paginate(&ctx, embeds).await?,
	}

	Ok(())
}
