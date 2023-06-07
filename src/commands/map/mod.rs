mod map_embed;
pub use map_embed::build_embed;

use super::autocomplete;
use crate::{Context, Error, Result};

/// Get detailed information on a map.
///
/// This command will fetch a bunch of useful information about a particular map. The information \
/// is a combination of the \
/// [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2), \
/// [n4vyn's](https://github.com/n4vyn) [KZ:GO API](https://kzgo.eu/) and my own \
/// [SchnoseAPI](https://github.com/Schnose/SchnoseAPI). If anything seems incorrect, feel free \
/// to report it.
#[poise::command(slash_command, on_error = "Error::global_handler")]
pub async fn map(
	ctx: Context<'_>,

	#[description = "Choose a map"]
	#[autocomplete = "autocomplete::map_name"]
	map: autocomplete::GlobalMap,
) -> Result<()> {
	ctx.defer().await?;

	ctx.send(|reply| reply.embed(|embed| build_embed(&ctx, embed, &map)))
		.await?;

	Ok(())
}
