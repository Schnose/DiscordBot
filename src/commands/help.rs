use std::{collections::BTreeMap, time::Duration};

use poise::serenity_prelude::{CollectComponentInteraction, InteractionResponseType};
use schnose_discord_bot::State;

use crate::{Context, Error, Result};

/// Help Menu
///
/// First of all, thank you for using this bot! I always appreciate suggestions and bug reports, \
/// so if you have anything, feel free to reach out via:
/// 1. `/report`
/// 2. DM to <@291585142164815873>
/// 3. an [issue on GitHub](https://github.com/Schnose/DiscordBot/issues)
///
/// This bot features most commands you already know from ingame such as
/// - `/pb`
/// - `/wr`
/// - `/maptop`
/// as well as a bunch of other utility commands.
///
/// To get started, type `/` and click on schnose's icon on the left to see all available \
/// commands, or scroll through this help menu. A lot of commands will have `mode` or `player` as \
/// a possible command argument (e.g. `/pb`) since the bot doesn't know _which_ PB to look up. \
/// It's gonna try to guess as best as it can, but to get the best results, you should save your \
/// `SteamID` and your preferred mode in the bot's database. You can do that with `/setsteam` and \
/// `/mode`. Those will then be used as fallback options.
#[poise::command(slash_command, on_error = "Error::global_handler")]
pub async fn help(ctx: Context<'_>) -> Result<()> {
	let commands = ctx
		.framework()
		.options
		.commands
		.iter()
		.filter_map(|command| {
			let name = command.name.clone();
			let description = command.description.clone()?;
			let help_text = command.help_text?();

			Some((name, (description, help_text)))
		})
		.collect::<BTreeMap<String, (String, String)>>();

	let ctx_id = ctx.id();

	ctx.send(|reply| {
		let (title, description) = commands
			.get("help")
			.expect("The `/help` command should have a help page.");

		reply
			.embed(|embed| {
				embed
					.color(ctx.color())
					.title(title)
					.description(description)
					.footer(|footer| ctx.footer(footer))
			})
			.components(|components| {
				components.create_action_row(|action_row| {
					action_row.create_select_menu(|select_menu| {
						select_menu
							.custom_id(ctx_id)
							.options(|menu_options| {
								for (name, (description, _)) in &commands {
									menu_options.create_option(|option| {
										option
											.label(format!("/{name}"))
											.value(name)
											.description(description)
									});
								}

								menu_options
							})
					})
				})
			})
	})
	.await?;

	while let Some(interaction) = CollectComponentInteraction::new(ctx)
		.filter(move |interaction| interaction.data.custom_id == ctx_id.to_string())
		.timeout(Duration::from_secs(600))
		.await
	{
		let choice = &interaction.data.values[0];

		interaction
			.create_interaction_response(ctx, |response| {
				response
					.kind(InteractionResponseType::UpdateMessage)
					.interaction_response_data(|data| {
						data.embed(|embed| {
							let (title, description) = commands.get(choice.as_str()).map_or(
								(format!("/{choice}"), String::new()),
								|(title, description)| {
									let title = format!("[/{choice}]: {title}");
									(title, description.to_owned())
								},
							);

							embed
								.color(ctx.color())
								.title(title)
								.description(description)
								.footer(|footer| ctx.footer(footer))
						})
					})
			})
			.await?;
	}

	Ok(())
}
