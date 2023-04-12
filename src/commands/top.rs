use {
	super::{
		custom_params::{ModeChoice, RuntypeChoice},
		pagination,
	},
	crate::{
		error::{Error, Result},
		state::{Context, StateContainer},
	},
	gokz_rs::{global_api, kzgo_api},
	poise::serenity_prelude::CreateEmbed,
};

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
#[tracing::instrument(skip(ctx), fields(user = ctx.author().tag()))]
#[poise::command(slash_command, ephemeral, on_error = "Error::handle")]
pub async fn top(
	ctx: Context<'_>,

	#[description = "KZT/SKZ/VNL"]
	#[rename = "mode"]
	mode_choice: Option<ModeChoice>,

	#[description = "TP/PRO"]
	#[rename = "runtype"]
	runtype_choice: Option<RuntypeChoice>,
) -> Result<()> {
	ctx.defer().await?;

	let mode = match mode_choice {
		Some(choice) => choice.into(),
		None => ModeChoice::figure_out(ctx.author_id().into(), &ctx).await,
	};

	let runtype = matches!(runtype_choice, Some(RuntypeChoice::TP));

	let top = global_api::get_wr_top(mode, runtype, 0..1, ctx.gokz_client())
		.await?
		.into_iter()
		.take(100)
		.collect::<Vec<_>>();

	let nr1_avatar = match top
		.first()
		.map(|player| player.steam_id)
	{
		None => String::new(),
		Some(steam_id) => kzgo_api::get_avatar(steam_id, ctx.gokz_client())
			.await
			.map(|user| user.avatar_url)
			.unwrap_or_default(),
	};

	let mut embeds = Vec::new();
	let mut temp = CreateEmbed::default();

	// How many entries per page
	let chunk_size = 12;
	let max_pages = (top.len() as f64 / chunk_size as f64).ceil() as u8;
	let mut place = 1;

	for (page_idx, players) in top.chunks(chunk_size).enumerate() {
		temp.color(ctx.color())
			.title(format!(
				"[{} {}] Top 100 World Record holders",
				mode.short(),
				if runtype { "TP" } else { "PRO" }
			))
			.url(format!("https://kzgo.eu/leaderboards?{}=", mode.short().to_lowercase()))
			.thumbnail(&nr1_avatar)
			.footer(|footer| {
				footer.text(format!("{} | Page {} / {}", ctx.schnose(), page_idx + 1, max_pages))
			});

		for player in players {
			let player_name = &player.player_name;

			temp.field(format!("{player_name} [#{place}]"), player.count, true);
			place += 1;
		}

		embeds.push(temp);
		temp = CreateEmbed::default();
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
