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
	std::path::PathBuf,
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
async fn schnosebot() -> ShuttleResult {
	let config_path: PathBuf = std::env::var("SCHNOSE_DISCORD_BOT_CONFIG_DIR")
		.unwrap_or_else(|_| String::from("./config.toml"))
		.into();

	let config_file = std::fs::read_to_string(config_path).expect(
		"Failed to read config file.
		 Use `SCHNOSE_DISCORD_BOT_CONFIG_DIR` if you want to specify a custom location.",
	);

	let config: Config = toml::from_str(&config_file).expect("Failed to parse config file.");

	let state = State::new(config).await;

	let framework_options = FrameworkOptions {
		owners: state
			.config
			.owners
			.iter()
			.copied()
			.map(Into::into)
			.collect(),
		prefix_options: PrefixFrameworkOptions { ignore_bots: true, ..Default::default() },
		commands: vec![
			commands::apistatus(),
			commands::bmaptop(),
			commands::bpb(),
			commands::btop(),
			commands::bwr(),
			commands::db(),
			commands::invite(),
			commands::map(),
			commands::maptop(),
			commands::mode(),
			commands::nocrouch(),
			commands::pb(),
			commands::ping(),
			commands::profile(),
			commands::random(),
		],
		event_handler: |ctx, event, framework_ctx, state| {
			Box::pin(event_handler::handle(ctx, event, framework_ctx, state))
		},
		..Default::default()
	};

	let token = match &state.config.environment {
		config::Environment::Development { discord_token, .. } => discord_token,
		config::Environment::Production { discord_token, .. } => discord_token,
	}
	.clone();

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

					match &state.config.environment {
						config::Environment::Development { guild_id, .. } => {
							let guild_id = GuildId(*guild_id);
							poise::builtins::register_in_guild(ctx, commands, guild_id)
								.await
								.expect("Failed to register commands for GuildID `{guild_id}`.");
						}
						config::Environment::Production { .. } => {
							poise::builtins::register_globally(ctx, commands)
								.await
								.expect("Failed to register commands globally.");
						}
					};

					for Command { name, .. } in commands {
						info!(
							"[{}] Successfully registered command `{}`.",
							state.config.environment, name,
						);
					}

					Ok(state)
				})
			}),
	))
}
