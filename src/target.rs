use {crate::error::Error, gokz_rs::SteamID, lazy_static::lazy_static, regex::Regex};

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Target {
	/// No target has been specified -> take the `UserID` of the user who triggered the command
	None { user_id: u64 },

	/// The user has @mention'd someone -> take the `UserID` from the mention
	Mention { user_id: u64 },

	/// The user has entered a [`SteamID`]
	SteamID { steam_id: SteamID },

	/// The user has entered _something_ -> we interpret it as a name
	Name { name: String },
}

impl std::fmt::Display for Target {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::None { user_id } => f.write_fmt(format_args!("<@{user_id}>")),
			Self::Mention { user_id } => f.write_fmt(format_args!("<@{user_id}>")),
			Self::SteamID { steam_id } => f.write_fmt(format_args!("{steam_id}")),
			Self::Name { name } => f.write_fmt(format_args!("{name}")),
		}
	}
}

lazy_static! {
	pub static ref MENTION_REGEX: Regex =
		Regex::new(r#"^<@[0-9]+>$"#).expect("Failed to compile regex.");
}

impl std::str::FromStr for Target {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s.is_empty() {
			return Err(Error::EmptyInput {
				expected: String::from("User identifier"),
			});
		}

		if let Ok(steam_id) = SteamID::new(s) {
			return Ok(Self::SteamID { steam_id });
		}

		if MENTION_REGEX.is_match(s) {
			let user_id = s
				.replace("<@", "")
				.replace('>', "")
				.parse::<u64>()
				.unwrap();

			return Ok(Self::Mention { user_id });
		}

		Ok(Self::Name { name: s.to_owned() })
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn empty() {
		let target = "";
		let parsed = target.parse::<Target>();
		assert_eq!(
			parsed,
			Err(Error::EmptyInput {
				expected: String::from("User identifier")
			})
		);
	}

	#[test]
	fn steam_id() {
		let steam_id = SteamID::from_id32(322356345);
		let steam_id_input = steam_id.to_string();
		let parsed = steam_id_input.parse::<Target>();
		assert_eq!(parsed, Ok(Target::SteamID { steam_id }));
	}

	#[test]
	fn mention() {
		let mention = "<@291585142164815873>";
		let parsed = mention.parse::<Target>();
		assert_eq!(parsed, Ok(Target::Mention { user_id: 291585142164815873 }));
	}

	#[test]
	fn name() {
		let mention = "AlphaKeks";
		let parsed = mention.parse::<Target>();
		assert_eq!(parsed, Ok(Target::Name { name: String::from(mention) }));
	}
}
