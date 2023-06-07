//! See `lib.rs`

#![warn(missing_debug_implementations, missing_docs, rust_2018_idioms)]
#![warn(clippy::style, clippy::complexity)]
#![deny(clippy::perf, clippy::correctness)]

mod commands;
mod events;

use crate::events::EventHandler;
use poise::{
	async_trait,
	serenity_prelude::{GatewayIntents, GuildId},
	Command, Framework, FrameworkOptions,
};
use schnose_discord_bot::GlobalState;
use shuttle_service::{SecretStore, Service};
use std::{collections::HashSet, net::SocketAddr, sync::Arc};

pub(crate) use schnose_discord_bot::*;

struct BotService {
	framework: Arc<Framework<schnose_discord_bot::GlobalState, schnose_discord_bot::Error>>,
}

type ShuttleResult<T> = std::result::Result<T, shuttle_service::Error>;

#[async_trait]
impl Service for BotService {
	#[tracing::instrument(skip(self))]
	async fn bind(self, _addr: SocketAddr) -> ShuttleResult<()> {
		tokio::select! {
			framework = self.framework.start_autosharded() => {
				framework.map_err(shuttle_service::CustomError::msg)?;
			}
		}

		Ok(())
	}
}

#[shuttle_runtime::main]
async fn discord_bot(
	#[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> ShuttleResult<BotService> {
	let token = secret_store
		.get("DISCORD_TOKEN")
		.ok_or("Missing secret `DISCORD_TOKEN`.")
		.map_err(shuttle_service::CustomError::msg)?;

	let intents = GatewayIntents::GUILDS
		| GatewayIntents::GUILD_MEMBERS
		| GatewayIntents::GUILD_MESSAGES
		| GatewayIntents::MESSAGE_CONTENT;

	let mut global_state = GlobalState::new(&secret_store)
		.await
		.map_err(shuttle_service::CustomError::msg)?;

	let framework = Framework::builder()
		.token(token)
		.intents(intents)
		.options(FrameworkOptions {
			owners: HashSet::from_iter([global_state.owner_id]),
			on_error: |err| Box::pin(async move { Error::global_handler(err).await }),
			pre_command: |ctx| {
				Box::pin(async move {
					let command = &ctx.command().name;
					let user_id = *ctx.author().id.as_u64();
					let channel_id = *ctx.channel_id().as_u64();
					let guild_name = match ctx.guild() {
						None => String::from("None"),
						Some(guild) => guild.name,
					};

					trace!(
						ctx.state(),
						r#"`/{command}` has been invoked.
- User: <@{user_id}>
- Channel: <#{channel_id}>
- Server: `{guild_name}`"#
					);
				})
			},
			post_command: |ctx| {
				Box::pin(async move {
					trace!(ctx.state(), "Done executing command.");
				})
			},
			reply_callback: /* Some(|_ctx, _reply| {}) */ None,
			event_handler: |ctx, event, framework_ctx, global_state| {
				Box::pin(async move {
					EventHandler::handle(ctx, event, framework_ctx, global_state).await
				})
			},
			commands: vec![
				commands::ping(),
				commands::invite(),
				commands::help(),
				commands::report(),
				commands::db(),
				commands::setsteam(),
				commands::mode(),
				commands::apistatus(),
				commands::map(),
				commands::wr(),
				commands::pb(),
				commands::maptop(),
				commands::bwr(),
				commands::bpb(),
				commands::bmaptop(),
				commands::nocrouch(),
				commands::random(),
				commands::top(),
				commands::btop(),
				commands::unfinished(),
			],
			..Default::default()
		})
		.setup(|ctx, _ready, framework| {
			Box::pin(async move {
				let http = Arc::clone(&ctx.http);
				global_state.http = Some(http);

				let commands = &framework.options().commands;
				let guild_id = secret_store
					.get("DEV_GUILD_ID")
					.ok_or(err!("Missing secret `DEV_GUILD_ID`."))?
					.parse()
					.map(GuildId)
					.map_err(|err| err!("Invalid GuildID: {err:?}"))?;

				poise::builtins::register_in_guild(&ctx.http, commands, guild_id).await?;

				let mut message = String::from("Registering commands:");

				for Command { name, .. } in commands {
					use std::fmt::Write;

					write!(&mut message, "\n- `/{name}`")
						.expect("Failed to append command to message");
				}

				info!(&global_state, "{message}");

				Ok(global_state)
			})
		})
		.build()
		.await
		.map_err(shuttle_service::CustomError::msg)?;

	Ok(BotService { framework })
}
