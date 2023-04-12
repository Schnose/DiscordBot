use {
	crate::{error::Result, state::Context},
	poise::serenity_prelude::{CollectComponentInteraction, CreateEmbed, InteractionResponseType},
	std::time::Duration,
};

pub async fn paginate(ctx: &Context<'_>, embeds: Vec<CreateEmbed>) -> Result<()> {
	let ctx_id = ctx.id();
	let prev_id = format!("{ctx_id}_prev");
	let next_id = format!("{ctx_id}_next");

	// Send first embed
	ctx.send(|reply| {
		reply
			.embed(|embed| {
				*embed = embeds[0].clone();
				embed
			})
			.components(|componends| {
				componends.create_action_row(|row| {
					row.create_button(|button| button.custom_id(&prev_id).label('◀'))
						.create_button(|button| button.custom_id(&next_id).label('▶'))
				})
			})
	})
	.await?;

	let mut current_page = 0;

	// Listen for button presses
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

		// Not the interaction we're looking for
		if interaction_id != &prev_id && interaction_id != &next_id {
			continue;
		}

		if interaction_id == &prev_id {
			// first page
			if current_page == 0 {
				// flip to last page
				current_page = embeds.len() - 1;
			} else {
				// flip 1 page back
				current_page -= 1;
			}
		} else {
			current_page += 1;
			// index is higher than allowed
			if current_page >= embeds.len() {
				// flip to first page
				current_page = 0;
			}
		}

		// Flip page
		interaction
			.create_interaction_response(ctx, |response| {
				response
					.kind(InteractionResponseType::UpdateMessage)
					.interaction_response_data(|data| data.set_embed(embeds[current_page].clone()))
			})
			.await?;
	}

	todo!()
}
