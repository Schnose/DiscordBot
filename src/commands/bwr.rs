use {
	super::{autocomplete, custom_params::ModeChoice},
	crate::{
		error::{Error, Result},
		state::{Context, StateContainer},
	},
	gokz_rs::global_api,
	schnosebot::time,
};

/// World record on a given bonus course.
///
// This command will fetch the world record on a particular bonus. You are required to specify a \
// `map` and may also specify the following options:
//
// - `mode`: `KZTimer` / `SimpleKZ` / `Vanilla`
//   - If you don't specify this, the bot will search the database for your UserID. If it can't \
//     find one, or you don't have a mode preference set, the command will fail. To save a mode \
//     preference in the database, see `/mode`.
// - `course`: this can be any integer between 1-255.
//   - If you either don't specify this, or put in `0`, the bot will default to `1`.
#[tracing::instrument(skip(ctx), fields(user = ctx.author().tag()))]
#[poise::command(slash_command, ephemeral, on_error = "Error::handle")]
pub async fn bwr(
	ctx: Context<'_>,

	#[description = "Choose a map"]
	#[rename = "map"]
	#[autocomplete = "autocomplete::map_name"]
	map_choice: String,

	#[description = "KZT/SKZ/VNL"]
	#[rename = "mode"]
	mode_choice: Option<ModeChoice>,

	#[description = "Which bonus?"]
	#[rename = "course"]
	course_choice: Option<u8>,
) -> Result<()> {
	ctx.defer().await?;

	let map = ctx.get_map(map_choice.clone())?;

	let mode = match mode_choice {
		Some(choice) => choice.into(),
		None => ModeChoice::figure_out(ctx.author_id().into(), &ctx).await,
	};

	let course = course_choice.unwrap_or(1).max(1);

	let tp_wr =
		global_api::get_wr(map.name.clone().into(), mode, true, course, ctx.gokz_client()).await;

	let pro_wr =
		global_api::get_wr(map.name.clone().into(), mode, false, course, ctx.gokz_client()).await;

	if tp_wr.is_err() && pro_wr.is_err() {
		return Err(Error::NoRecords);
	}

	let (tp_time, tp_links) = match &tp_wr {
		Err(_) => (String::from("😔"), None),
		Ok(wr) => {
			let time = time::format(wr.time);
			let teleports = match wr.teleports {
				0 => String::new(),
				1 => String::from("(1 TP)"),
				n => format!("({n} TPs)"),
			};

			let player_name =
				format!("[{}](https://kzgo.eu/players/{})", wr.player_name, wr.steam_id);

			(
				format!("{time} {teleports}\n> by {player_name}"),
				Some((wr.replay_view_link(), wr.replay_download_link())),
			)
		}
	};

	let (pro_time, pro_links) = match &pro_wr {
		Err(_) => (String::from("😔"), None),
		Ok(wr) => {
			let time = time::format(wr.time);
			let player_name =
				format!("[{}](https://kzgo.eu/players/{})", wr.player_name, wr.steam_id);

			(
				format!("{time} \n> by {player_name}"),
				Some((wr.replay_view_link(), wr.replay_download_link())),
			)
		}
	};

	ctx.send(|reply| {
		reply.embed(|embed| {
			embed
				.color(ctx.color())
				.title(format!("[WR] {map_name} B{course}", map_name = &map.name))
				.url(format!(
					"{link}?{mode}=&bonus={course}",
					link = map.kzgo_link(),
					mode = mode.short().to_lowercase()
				))
				.thumbnail(map.thumbnail())
				.description(
					crate::utils::format_replay_links(tp_links, pro_links).unwrap_or_default(),
				)
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
