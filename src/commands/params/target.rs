use crate::{Context, State};
use gokz_rs::{PlayerIdentifier, SteamID};
use lazy_static::lazy_static;
use poise::{
	async_trait,
	serenity_prelude::{
		json::Value as JsonValue, CommandOptionType, Context as SerenityContext,
		CreateApplicationCommandOption, UserId,
	},
	ApplicationCommandOrAutocompleteInteraction, SlashArgError,
};
use regex::Regex;

lazy_static! {
	pub static ref MENTION_REGEX: Regex =
		Regex::new(r#"^<@[0-9]+>$"#).expect("Failed to compile regex.");
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Target {
	/// The user has specified no target => take their UserID
	None(UserId),

	/// The user @mention'd someone => take that UserID
	Mention(UserId),

	/// The user put in a [`SteamID`]
	SteamID(SteamID),

	/// The user put in something else => consider it a name
	Name(String),
}

#[async_trait]
impl poise::SlashArgument for Target {
	async fn extract(
		_ctx: &SerenityContext,
		interaction: ApplicationCommandOrAutocompleteInteraction<'_>,
		value: &JsonValue,
	) -> Result<Self, SlashArgError> {
		if let Some(id) = value.as_u64() {
			if let Ok(steam_id) = SteamID::from_id64(id) {
				return Ok(Self::SteamID(steam_id));
			}

			return Err(SlashArgError::Parse {
				error: "Invalid player id. Please @mention someone or provide a valid SteamID / player name.".into(),
				input: value.to_string(),
			});
		}

		if let Some(input) = value.as_str() {
			if let Ok(steam_id) = SteamID::new(input) {
				return Ok(Self::SteamID(steam_id));
			}

			if MENTION_REGEX.is_match(input) {
				let user_id = input
					.replace("<@", "")
					.replace('>', "")
					.parse::<u64>()
					.map_err(|_| SlashArgError::Parse {
						error: format!("Invalid mention: `{input}`").into(),
						input: input.to_owned(),
					})?;

				return Ok(Self::Mention(UserId(user_id)));
			}

			return Ok(Self::Name(input.to_owned()));
		}

		Ok(Self::None(interaction.user().id))
	}

	fn create(builder: &mut CreateApplicationCommandOption) {
		builder
			.name("player")
			.kind(CommandOptionType::String)
			.required(false);
	}
}

impl Target {
	pub async fn into_player(self, ctx: &Context<'_>) -> PlayerIdentifier {
		match self {
			Self::None(user_id) | Self::Mention(user_id) => {
				if let Ok(Some(user)) = ctx.get_user_by_id(user_id).await {
					if let Some(steam_id) = user.steam_id {
						return steam_id.into();
					}

					return user.name.into();
				}

				if let Some(guild) = ctx.guild() {
					if let Ok(member) = guild.member(ctx, user_id).await {
						return member.user.name.into();
					}
				}

				ctx.author().name.clone().into()
			}
			Self::SteamID(steam_id) => steam_id.into(),
			Self::Name(name) => name.into(),
		}
	}
}
