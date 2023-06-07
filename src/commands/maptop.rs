use super::{autocomplete, params};
use crate::{Context, Error, Result, State};
use gokz_rs::global_api;
use poise::serenity_prelude::CreateEmbed;

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
#[poise::command(slash_command, on_error = "Error::global_handler")]
pub async fn maptop(
	ctx: Context<'_>,

	#[description = "Choose a map"]
	#[autocomplete = "autocomplete::map_name"]
	map: autocomplete::GlobalMap,

	#[description = "KZT/SKZ/VNL"] mode: Option<params::Mode>,

	#[description = "TP/PRO"] runtype: Option<params::Runtype>,
) -> Result<()> {
	ctx.defer().await?;

	let course = 0;
	_maptop(ctx, map, mode, runtype, course).await?;

	Ok(())
}

/// Top 100 records on a bonus.
///
/// This command will fetch the top 100 (or less, if there are less than 100 completions) records \
/// on a particular bonus. You are required to specify a `map` and may also specify the \
/// following options:
///
/// - `mode`: `KZTimer` / `SimpleKZ` / `Vanilla`
///   - If you don't specify this, the bot will search the database for your UserID. If it can't \
///     find one, or you don't have a mode preference set, the command will fail. To save a mode \
///     preference in the database, see `/mode`.
/// - `runtype`: `TP` / `PRO`
///   - If you don't specify this, the bot will default to `PRO`.
/// - `course`: this can be any integer between 1-255.
///   - If you either don't specify this, or put in `0`, the bot will default to `1`.
#[poise::command(slash_command, on_error = "Error::global_handler")]
pub async fn bmaptop(
	ctx: Context<'_>,

	#[description = "Choose a map"]
	#[autocomplete = "autocomplete::map_name"]
	map: autocomplete::GlobalMap,

	#[description = "KZT/SKZ/VNL"] mode: Option<params::Mode>,

	#[description = "TP/PRO"] runtype: Option<params::Runtype>,

	#[description = "Which bonus?"]
	#[min = 1]
	#[max = 100]
	course: Option<u8>,
) -> Result<()> {
	ctx.defer().await?;

	let course = course.unwrap_or(1);
	_maptop(ctx, map, mode, runtype, course).await?;

	Ok(())
}

async fn _maptop(
	ctx: Context<'_>,
	map: autocomplete::GlobalMap,
	mode: Option<params::Mode>,
	runtype: Option<params::Runtype>,
	course: u8,
) -> Result<()> {
	let mode = params::Mode::parse_param(mode, &ctx).await?;
	let runtype = runtype.unwrap_or_default();
	let maptop =
		global_api::get_maptop(map.id.into(), mode, runtype.into(), course, ctx.gokz_client())
			.await?;

	if maptop.is_empty() {
		return Err(Error::NoRecords);
	}

	let mut embeds = Vec::new();
	let mut temp = CreateEmbed::default();

	let records_per_page = 12;
	let max_pages = (maptop.len() as f64 / records_per_page as f64).ceil() as u8;
	let mut place = 1;

	for (page_idx, records) in maptop
		.chunks(records_per_page)
		.enumerate()
	{
		let mut title = format!("[{} {:?}] Top 100 on {}", mode.short(), runtype, map.name);

		if course > 0 {
			use std::fmt::Write;
			write!(&mut title, " B{}", course)?;
		};

		let url = format!("{}?{}=", map.kzgo_link(), mode.short().to_lowercase());
		let thumbnail = map.thumbnail();

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

		for record in records {
			let title = format!("{player} [#{place}]", player = record.player_name);
			let time = schnosebot::time::format(record.time);
			let teleports = match record.teleports {
				0 => String::new(),
				1 => String::from(" (1 TP)"),
				n => format!(" ({n} TPs)"),
			};

			temp.field(title, format!("{time}{teleports}"), true);
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
