use gokz_rs::{global_api, Mode, SteamID};

type Record = std::result::Result<global_api::Record, gokz_rs::Error>;
type Links = (Option<String>, Option<String>);

/// Takes two records and formats them into embed descriptions and replay links
pub async fn parse_records(
	tp: &Record,
	pro: &Record,
	fetch_places: Option<&gokz_rs::Client>,
) -> ((String, Option<Links>), (String, Option<Links>)) {
	let tp = parse_record(tp, fetch_places).await;
	let pro = parse_record(pro, fetch_places).await;
	(tp, pro)
}

/// Takes a record and formats it into an embed description and replay links
pub async fn parse_record(
	rec: &Record,
	fetch_place: Option<&gokz_rs::Client>,
) -> (String, Option<Links>) {
	match rec {
		Err(_) => (String::from("😔"), None),
		Ok(rec) => {
			let time = schnosebot::time::format(rec.time);
			let teleports = match rec.teleports {
				0 => String::new(),
				1 => String::from(" (1 TP)"),
				n => format!(" ({n} TPs)"),
			};

			let place = 'place: {
				if let Some(client) = fetch_place {
					if let Ok(place) = global_api::get_place(rec.id, client).await {
						break 'place format!("[#{place}] ");
					};
				}

				String::new()
			};

			let player_name = format!(
				"[{name}](https://kzgo.eu/players/{steam_id}?{mode}=)",
				name = rec.player_name,
				steam_id = rec.steam_id,
				mode = rec.mode.short().to_lowercase(),
			);

			let formatted = format!("{place}{time}{teleports}\n> by {player_name}");
			let links = Some((rec.replay_view_link(), rec.replay_download_link()));

			(formatted, links)
		}
	}
}

/// Takes a set of replay links and builds an embed description
pub fn format_replay_links(tp: Option<Links>, pro: Option<Links>) -> Option<String> {
	let tp =
		if let Some((Some(view), Some(download))) = tp { Some((view, download)) } else { None };

	let pro =
		if let Some((Some(view), Some(download))) = pro { Some((view, download)) } else { None };

	match (tp, pro) {
		(Some((tp_view, tp_download)), Some((pro_view, pro_download))) => {
			Some(format!("TP Replay: [View Online]({tp_view}) | [Download]({tp_download})\nPRO Replay: [View Online]({pro_view}) | [Download]({pro_download})"))
		}
		(Some((tp_view, tp_download)), None) => {
			Some(format!("TP Replay: [View Online]({tp_view}) | [Download]({tp_download})"))
		}
		(None, Some((pro_view, pro_download))) => {
			Some(format!("PRO Replay: [View Online]({pro_view}) | [Download]({pro_download})"))
		}
		(None, None) => {
			None
		}
	}
}

/// Parses player profile links out of records for an embed description
pub fn get_player_info(tp: Record, pro: Record) -> Option<String> {
	let format = |steam_id: SteamID, mode: Mode| {
		format!(
			"Player: [KZ:GO](https://kzgo.eu/players/{steam_id}?{mode}=) | [Steam](https://steamcommunity.com/profiles/{id64})",
			steam_id = steam_id,
			mode = mode.short().to_lowercase(),
			id64 = steam_id.as_id64(),
		)
	};

	if let Ok(rec) = tp {
		return Some(format(rec.steam_id, rec.mode));
	} else if let Ok(rec) = pro {
		return Some(format(rec.steam_id, rec.mode));
	}

	None
}
