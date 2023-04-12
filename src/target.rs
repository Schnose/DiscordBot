use gokz_rs::SteamID;

#[derive(Debug, Clone)]
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
