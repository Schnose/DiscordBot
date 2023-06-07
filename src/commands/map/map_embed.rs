use crate::{Context, State};
use poise::serenity_prelude::CreateEmbed;
use schnosebot::global_map::GlobalMap;

pub fn build_embed<'embed>(
	ctx: &'_ Context<'_>,
	embed: &'embed mut CreateEmbed,
	map: &'_ GlobalMap,
) -> &'embed mut CreateEmbed {
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
		.collect::<Vec<_>>()
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
		.footer(|footer| ctx.footer(footer))
}
