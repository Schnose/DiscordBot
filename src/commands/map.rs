use schnosebot::global_map::GlobalMap;
use serenity::builder::CreateEmbed;

use {
	super::autocomplete,
	crate::{
		error::{Error, Result},
		state::{Context, StateContainer},
	},
	itertools::Itertools,
};

/// Get detailed information on a map.
///
/// This command will fetch a bunch of useful information about a particular map. The information \
/// is a combination of the \
/// [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2), \
/// [n4vyn's](https://github.com/n4vyn) [KZ:GO API](https://kzgo.eu/) and my own \
/// [SchnoseAPI](https://github.com/Schnose/SchnoseAPI). If anything seems incorrect, feel free \
/// to report it.
#[tracing::instrument(skip(ctx), fields(user = ctx.author().tag()))]
#[poise::command(slash_command, ephemeral, on_error = "Error::handle")]
pub async fn map(
	ctx: Context<'_>,

	#[description = "Choose a map"]
	#[rename = "map"]
	#[autocomplete = "autocomplete::map_name"]
	map_choice: String,
) -> Result<()> {
	ctx.defer().await?;

	let map = ctx.get_map(map_choice.clone())?;

	ctx.send(|reply| reply.embed(|embed| build_map_embed(&ctx, embed, &map)))
		.await?;

	Ok(())
}

pub(super) fn build_map_embed<'e>(
	ctx: &'_ Context<'_>,
	embed: &'e mut CreateEmbed,
	map: &'_ GlobalMap,
) -> &'e mut CreateEmbed {
	let mappers = map
		.mappers
		.iter()
		.map(|mapper| {
			format!(
				"[{}](https://steamcommunity.com/profiles/{})",
				mapper.name,
				mapper.steam_id.as_id64()
			)
		})
		.join(", ");

	let kzt_filer = if map.kzt { "✅" } else { "❌" };
	let skz_filer = if map.skz { "✅" } else { "❌" };
	let vnl_filer = if map.vnl { "✅" } else { "❌" };

	embed
		.color(ctx.color())
		.title(&map.name)
		.url(&map.kzgo_link())
		.thumbnail(&map.thumbnail())
		.description(format!(
			r#"
🡆 Tier: {} ({})
🡆 Mapper(s): {}
🡆 Bonuses: {}
🡆 Last Updated: {}

🡆 Filters:
					"#,
			map.tier as u8,
			map.tier,
			mappers,
			map.courses.len() - 1,
			map.updated_on.format("%d/%m/%Y"),
		))
		.field("KZT", kzt_filer, true)
		.field("SKZ", skz_filer, true)
		.field("VNL", vnl_filer, true)
}
