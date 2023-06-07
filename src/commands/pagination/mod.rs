use std::time::Duration;

use crate::{Context, Result};
use poise::serenity_prelude::{CollectComponentInteraction, CreateEmbed, InteractionResponseType};

/// Takes a list of embeds and will send a paginated reply
pub async fn paginate(ctx: &Context<'_>, embeds: &[CreateEmbed]) -> Result<()> {
	if embeds.is_empty() {
		return Ok(());
	}

	let ctx_id = ctx.id();
	let prev_id = format!("{ctx_id}_prev");
	let next_id = format!("{ctx_id}_next");

	ctx.send(|reply| {
		reply
			.embed(|embed| {
				*embed = embeds[0].clone();
				embed
			})
			.components(|components| {
				components.create_action_row(|action_row| {
					action_row
						.create_button(|button| button.custom_id(&prev_id).label('◀'))
						.create_button(|button| button.custom_id(&next_id).label('▶'))
				})
			})
	})
	.await?;

	let mut current_page = 0;

	while let Some(interaction) = CollectComponentInteraction::new(ctx)
		.filter(move |interaction| {
			interaction
				.data
				.custom_id
				.starts_with(&ctx_id.to_string())
		})
		.timeout(Duration::from_secs(600))
		.await
	{
		let interaction_id = &interaction.data.custom_id;

		if interaction_id != &prev_id && interaction_id != &next_id {
			continue;
		}

		match (interaction_id == &prev_id, current_page == 0) {
			(true, true) => current_page = embeds.len() - 1,
			(true, false) => current_page -= 1,
			(false, _) => {
				current_page += 1;

				if current_page >= embeds.len() {
					current_page = 0;
				}
			}
		};

		interaction
			.create_interaction_response(ctx, |response| {
				response
					.kind(InteractionResponseType::UpdateMessage)
					.interaction_response_data(|data| data.set_embed(embeds[current_page].clone()))
			})
			.await?;
	}

	Ok(())
}
