use {
	super::{
		autocomplete,
		custom_params::{ModeChoice, RuntypeChoice},
		pagination,
	},
	crate::{
		error::{Error, Result},
		state::{Context, StateContainer},
	},
	gokz_rs::global_api,
	poise::serenity_prelude::CreateEmbed,
	schnosebot::time,
};

/// Top 100 records on a map.
///
/// This command will fetch the top 100 (or less, if there are less than 100 completions) records \
/// on a particular map. You are required to specify a `map` and may also specify the \
/// following options:
///
/// - `mode`: `KZTimer` / `SimpleKZ` / `Vanilla`
///   - If you don't specify this, the bot will search the database for your UserID. If it can't \
///     find one, or you don't have a mode preference set, the command will fail. To save a mode \
///     preference in the database, see `/mode`.
/// - `runtype`: `TP` / `PRO`
///   - If you don't specify this, the bot will default to `PRO`.
#[tracing::instrument(skip(ctx), fields(user = ctx.author().tag()))]
#[poise::command(slash_command, ephemeral, on_error = "Error::handle")]
pub async fn maptop(
	ctx: Context<'_>,

	#[description = "Choose a map"]
	#[rename = "map"]
	#[autocomplete = "autocomplete::map_name"]
	map_choice: String,

	#[description = "KZT/SKZ/VNL"]
	#[rename = "mode"]
	mode_choice: Option<ModeChoice>,

	#[description = "TP/PRO"]
	#[rename = "runtype"]
	runtype_choice: Option<RuntypeChoice>,
) -> Result<()> {
	ctx.defer().await?;

	let map = ctx
		.get_map(map_choice.clone())
		.ok_or(Error::MapNotGlobal { input: map_choice })?;

	let mode = match mode_choice {
		Some(choice) => choice.into(),
		None => ModeChoice::figure_out(ctx.author_id().into(), &ctx).await,
	};

	let runtype = matches!(runtype_choice, Some(RuntypeChoice::TP));

	let maptop =
		global_api::get_maptop(map.name.clone().into(), mode, runtype, 0, ctx.gokz_client())
			.await?;

	let mut embeds = Vec::new();
	let mut temp = CreateEmbed::default();

	// How many entries per page
	let chunk_size = 12;
	let max_pages = (maptop.len() as f64 / chunk_size as f64).ceil() as u8;
	let mut place = 1;

	for (page_idx, records) in maptop.chunks(chunk_size).enumerate() {
		temp.color(ctx.color())
			.title(format!(
				"[{} {}] Top 100 records on {}",
				mode.short(),
				if runtype { "TP" } else { "PRO" },
				&map.name
			))
			.url(format!("{}?{}=", map.kzgo_link(), mode.short().to_lowercase()))
			.thumbnail(map.thumbnail())
			.footer(|footer| {
				footer.text(format!("{} | Page {} / {}", ctx.schnose(), page_idx + 1, max_pages))
			});

		for record in records {
			let title = format!("{} [#{}]", record.player_name, place);
			let time = time::format(record.time);
			let teleports = match record.teleports {
				0 => String::new(),
				1 => String::from("(1 TP)"),
				n => format!("({n} TPs)"),
			};

			temp.field(title, format!("{time} {teleports}"), true);
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
