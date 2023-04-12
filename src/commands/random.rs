use {
	super::custom_params::TierChoice,
	crate::{
		error::{Error, Result},
		state::{Context, StateContainer},
	},
	rand::Rng,
};

/// Get a random map name from the global map pool.
///
/// This command will simply select a random map from the global map pool. You may specify a \
/// `tier` if you want to.
#[tracing::instrument(skip(ctx), fields(user = ctx.author().tag()))]
#[poise::command(slash_command, ephemeral, on_error = "Error::handle")]
pub async fn random(
	ctx: Context<'_>,

	#[description = "Filter by map difficulty"]
	#[rename = "tier"]
	tier_choice: Option<TierChoice>,
) -> Result<()> {
	ctx.defer().await?;

	let mut filtered_maps = ctx
		.maps()
		.iter()
		.filter(|map| tier_choice.map_or(true, |tier| map.tier as u8 == tier as u8))
		.collect::<Vec<_>>();

	let rng = rand::thread_rng().gen_range(0..filtered_maps.len());

	let map = filtered_maps.remove(rng);

	let mapper = match &map.mapper_steam_id {
		None => map.mapper_name.clone(),
		Some(steam_id) => format!(
			"[{}](https://steamcommunity.com/profiles/{})",
			map.mapper_name,
			steam_id.as_id64()
		),
	};

	let kzt_filer = if map.kzt { "✅" } else { "❌" };
	let skz_filer = if map.skz { "✅" } else { "❌" };
	let vnl_filer = if map.vnl { "✅" } else { "❌" };

	ctx.send(|reply| {
		reply.embed(|embed| {
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
					mapper,
					map.courses.len() - 1,
					map.updated_on.format("%d/%m/%Y"),
				))
				.field("KZT", kzt_filer, true)
				.field("SKZ", skz_filer, true)
				.field("VNL", vnl_filer, true)
		})
	})
	.await?;

	Ok(())
}
