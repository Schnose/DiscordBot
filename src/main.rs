//! This is a discord bot for CS:GO KZ using [serenity.rs](https://github.com/serenity-rs/serenity)
//! and [poise](https://github.com/serenity-rs/poise).
//!
//! It talks to different APIs and its own Database to provide useful functionality that you would
//! otherwise only get ingame or from a website like [KZ:GO](https://kzgo.eu). This functionality mostly
//! revolves around `/` commands, including:
//! - `/pb`
//! - `/wr`
//! - `/maptop`
//! - `/recent`
//! - `/profile`
//!
//! and many more! For a full list check out the [Wiki](https://github.com/Schnose/DiscordBot/wiki).
//! I am running a public instance that you can invite to your server via [this link](https://discord.com/oauth2/authorize?client_id=940308056451973120&permissions=327744&scope=bot%20applications.commands).

#![warn(missing_debug_implementations, missing_docs, rust_2018_idioms)]
#![warn(clippy::style, clippy::complexity, clippy::cognitive_complexity)]
#![deny(clippy::perf, clippy::correctness)]

use {
	crate::{
		config::Config,
		shuttle_integration::{SchnoseBot, ShuttleResult},
		state::State,
	},
	poise::{
		serenity_prelude::{GatewayIntents, GuildId},
		Command, Framework, FrameworkOptions, PrefixFrameworkOptions,
	},
	shuttle_secrets::SecretStore,
	std::collections::HashSet,
	tracing::info,
};

mod commands;
mod config;
mod database;
mod error;
mod event_handler;
mod shuttle_integration;
mod state;
mod target;
mod utils;

#[shuttle_runtime::main]
async fn schnosebot(#[shuttle_secrets::Secrets] secret_store: SecretStore) -> ShuttleResult {
	let config = Config::new(&secret_store);
	let state = State::new(config).await;

	let framework_options = FrameworkOptions {
		owners: HashSet::from_iter([state.config.owner_id.into()]),
		prefix_options: PrefixFrameworkOptions { ignore_bots: true, ..Default::default() },
		commands: vec![
			commands::apistatus(),
			commands::bmaptop(),
			commands::bpb(),
			commands::btop(),
			commands::bwr(),
			commands::db(),
			commands::help(),
			commands::invite(),
			commands::map(),
			commands::maptop(),
			commands::mode(),
			commands::nocrouch(),
			commands::pb(),
			commands::ping(),
			commands::profile(),
			commands::random(),
			commands::recent(),
			commands::report(),
			commands::setsteam(),
			commands::top(),
			commands::unfinished(),
			commands::wr(),
		],
		event_handler: |ctx, event, framework_ctx, state| {
			Box::pin(event_handler::handle(ctx, event, framework_ctx, state))
		},
		..Default::default()
	};

	let token = state.config.discord_token.clone();

	let intents = GatewayIntents::GUILDS
		| GatewayIntents::GUILD_MEMBERS
		| GatewayIntents::GUILD_MESSAGES
		| GatewayIntents::MESSAGE_CONTENT;

	Ok(SchnoseBot::new(
		Framework::builder()
			.options(framework_options)
			.token(token)
			.intents(intents)
			.setup(move |ctx, _, framework| {
				Box::pin(async move {
					let commands = &framework.options().commands;

					match state.config.guild_id {
						Some(guild_id) => {
							let guild_id = GuildId(guild_id);
							poise::builtins::register_in_guild(ctx, commands, guild_id)
								.await
								.expect("Failed to register commands for GuildID `{guild_id}`.");
						}
						None => {
							poise::builtins::register_globally(ctx, commands)
								.await
								.expect("Failed to register commands globally.");
						}
					}

					for Command { name, .. } in commands {
						info!(
							"[{}] Successfully registered command `{}`.",
							if state.config.guild_id.is_some() { "DEV" } else { "PROD" },
							name,
						);
					}

					Ok(state)
				})
			}),
	))
}
