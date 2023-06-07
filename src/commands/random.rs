use super::{map::build_embed, params};
use crate::{Context, Error, Result, State};
use rand::Rng;

/// Get a random map name from the global map pool.
///
/// This command will simply select a random map from the global map pool. You may specify a \
/// `tier` if you want to.
#[poise::command(slash_command, on_error = "Error::global_handler")]
pub async fn random(
	ctx: Context<'_>,

	#[description = "Fitler by difficulty"] tier: Option<params::Tier>,
) -> Result<()> {
	ctx.defer().await?;

	let filtered_maps = ctx
		.global_maps()
		.iter()
		.filter(|map| tier.map_or(true, |tier| map.tier as u8 == tier as u8))
		.collect::<Vec<_>>();

	let rng = rand::thread_rng().gen_range(0..filtered_maps.len());
	let map = filtered_maps[rng];

	ctx.send(|reply| reply.embed(|embed| build_embed(&ctx, embed, map)))
		.await?;

	Ok(())
}
