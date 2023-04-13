use {
	super::{autocomplete, custom_params::ModeChoice},
	crate::{
		error::{Error, Result},
		state::{Context, StateContainer},
		target::Target,
	},
	gokz_rs::{global_api, PlayerIdentifier},
	schnosebot::time,
};

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
#[tracing::instrument(skip(ctx), fields(user = ctx.author().tag()))]
#[poise::command(slash_command, ephemeral, on_error = "Error::handle")]
pub async fn pb(
	ctx: Context<'_>,

	#[description = "Choose a map"]
	#[rename = "map"]
	#[autocomplete = "autocomplete::map_name"]
	map_choice: String,

	#[description = "KZT/SKZ/VNL"]
	#[rename = "mode"]
	mode_choice: Option<ModeChoice>,

	#[description = "The player you want to target."]
	#[rename = "player"]
	target: Option<String>,
) -> Result<()> {
	ctx.defer().await?;

	let map = ctx
		.get_map(map_choice.clone())
		.ok_or(Error::MapNotGlobal { input: map_choice })?;

	let mode = match mode_choice {
		Some(choice) => choice.into(),
		None => ModeChoice::figure_out(ctx.author_id().into(), &ctx).await,
	};

	let target: Target = match target {
		None => ctx.author_id().into(),
		Some(target) => target.parse()?,
	};

	let player = target.into_player(&ctx).await;

	let tp_pb = global_api::get_pb(
		player.clone(),
		map.name.clone().into(),
		mode,
		true,
		0,
		ctx.gokz_client(),
	)
	.await;

	let pro_pb = global_api::get_pb(
		player.clone(),
		map.name.clone().into(),
		mode,
		false,
		0,
		ctx.gokz_client(),
	)
	.await;

	let mut player_name = match player {
		PlayerIdentifier::Name(player_name) => player_name.clone(),
		_ => String::from("unknown"),
	};

	let mut player_steam_id = None;

	if tp_pb.is_err() && pro_pb.is_err() {
		return Err(Error::NoRecords);
	}

	let (tp_time, tp_links) = match &tp_pb {
		Err(_) => (String::from("😔"), None),
		Ok(pb) => {
			player_name = pb.player_name.clone();
			player_steam_id = Some(pb.steam_id);

			let place = global_api::get_place(pb.id, ctx.gokz_client())
				.await
				.map(|place| format!("[#{place}]"))
				.unwrap_or_default();

			let time = time::format(pb.time);
			let teleports = match pb.teleports {
				0 => String::new(),
				1 => String::from("(1 TP)"),
				n => format!("({n} TPs)"),
			};

			(
				format!("{time} {place} {teleports}"),
				Some((pb.replay_view_link(), pb.replay_download_link())),
			)
		}
	};

	let (pro_time, pro_links) = match &pro_pb {
		Err(_) => (String::from("😔"), None),
		Ok(pb) => {
			player_name = pb.player_name.clone();
			player_steam_id = Some(pb.steam_id);

			let place = global_api::get_place(pb.id, ctx.gokz_client())
				.await
				.map(|place| format!("[#{place}]"))
				.unwrap_or_default();

			let time = time::format(pb.time);

			(format!("{time} {place}"), Some((pb.replay_view_link(), pb.replay_download_link())))
		}
	};

	let player_links = player_steam_id
		.map(|steam_id| {
			format!(
				"Player: [KZ:GO](https://kzgo.eu/players/{}?{}=) | [Steam](https://steamcommunity.com/profiles/{})",
				steam_id, mode.short().to_lowercase(), steam_id.as_id64()
			)
		})
		.unwrap_or_default();

	ctx.send(|reply| {
		reply.embed(|embed| {
			embed
				.color(ctx.color())
				.title(format!("[PB] {player_name} on {map_name}", map_name = &map.name))
				.url(format!(
					"{link}?{mode}=",
					link = map.kzgo_link(),
					mode = mode.short().to_lowercase()
				))
				.thumbnail(map.thumbnail())
				.description(format!(
					"{}\n\n{}",
					player_links,
					crate::utils::format_replay_links(tp_links, pro_links).unwrap_or_default(),
				))
				.field("TP", tp_time, true)
				.field("PRO", pro_time, true)
				.footer(|footer| {
					footer
						.text(format!("{} | Mode: {}", ctx.schnose(), mode.short()))
						.icon_url(ctx.icon_url())
				})
		})
	})
	.await?;

	Ok(())
}
