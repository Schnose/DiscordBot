use crate::{Context, State};
use poise::{
	async_trait,
	serenity_prelude::{
		json::Value as JsonValue, CommandOptionType, Context as SerenityContext,
		CreateApplicationCommandOption,
	},
	ApplicationCommandOrAutocompleteInteraction, SlashArgError,
};
use schnosebot::global_map;
use serde::Serialize;
use std::{
	fmt::Display,
	ops::{Deref, DerefMut},
};

#[derive(Debug, Clone)]
pub struct GlobalMap(pub global_map::GlobalMap);

impl Serialize for GlobalMap {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		self.name.serialize(serializer)
	}
}

impl Display for GlobalMap {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0.name)
	}
}

impl Deref for GlobalMap {
	type Target = global_map::GlobalMap;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for GlobalMap {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

#[async_trait]
impl poise::SlashArgument for GlobalMap {
	async fn extract(
		_ctx: &SerenityContext,
		_interaction: ApplicationCommandOrAutocompleteInteraction<'_>,
		value: &JsonValue,
	) -> Result<Self, SlashArgError> {
		let map_name = value
			.as_str()
			.ok_or(SlashArgError::Parse {
				error: "Expected map name.".into(),
				input: value.to_string(),
			})?;

		let global_maps = schnose_discord_bot::GLOBAL_MAPS
			.get()
			.ok_or(SlashArgError::Parse {
				error: "Map cache not ready yet.".into(),
				input: value.to_string(),
			})?;

		let map = global_map::GlobalMap::fuzzy_search(global_maps, map_name.to_owned()).ok_or(
			SlashArgError::Parse {
				error: format!("`{map_name}` is not a valid map name.").into(),
				input: value.to_string(),
			},
		)?;

		Ok(GlobalMap(map))
	}

	fn create(builder: &mut CreateApplicationCommandOption) {
		builder
			.name("map")
			.kind(CommandOptionType::String)
			.required(true)
			.set_autocomplete(true);
	}
}

pub async fn map_name<'ctx>(
	ctx: Context<'ctx>,
	input: &'ctx str,
) -> impl futures::Stream<Item = GlobalMap> + 'ctx {
	futures::stream::iter(
		global_map::GlobalMap::fuzzy_match(input, ctx.global_maps().as_slice())
			.into_iter()
			.map(GlobalMap),
	)
}
