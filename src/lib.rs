//! This is a discord bot for CS:GO KZ built with [poise](https://docs.rs/poise) and hosted on
//! [shuttle.rs](https://shuttle.rs/).
//!
//! It talks to different APIs and its own Database to provide useful functionality that you would
//! otherwise only get ingame or from a website like [KZ:GO](https://kzgo.eu). This functionality
//! mostly revolves around `/` commands, including:
//! - `/pb`
//! - `/wr`
//! - `/maptop`
//!
//! and many more! For a full list check out the [Wiki](https://github.com/Schnose/DiscordBot/wiki).
//! I am running a public instance that you can invite to your server via [this link](https://discord.com/oauth2/authorize?client_id=940308056451973120&permissions=327744&scope=bot%20applications.commands).

#![warn(missing_debug_implementations, missing_docs, rust_2018_idioms)]
#![warn(clippy::style, clippy::complexity)]
#![deny(clippy::perf, clippy::correctness)]

/// The crate's global error type.
///
/// Every fallible function except for main should return [`error::Result`].
mod error;
pub use error::{Error, Result};

/// Custom [`tracing`] wrapper
pub mod logging;

/// The bot's global state. This hold configuration options, secrets, and other "global"
/// information the bot should always have access to.
mod global_state;
pub use global_state::{GlobalState, State, GLOBAL_MAPS};

/// Type alias for convenience.
pub type Context<'ctx> = poise::Context<'ctx, GlobalState, Error>;

/// Contains all types regarding the bot's database.
pub mod database;

/// Contains utility functions to parse GlobalAPI data
pub mod global_api;
