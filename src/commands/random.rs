use {
	super::{custom_params::TierChoice, map::build_map_embed},
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

	ctx.send(|reply| reply.embed(|embed| build_map_embed(&ctx, embed, map)))
		.await?;

	Ok(())
}
