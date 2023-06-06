use crate::{info, trace};
use poise::{
	serenity_prelude::{Activity, Context as SerenityContext},
	Event, FrameworkContext,
};

pub struct EventHandler;

impl EventHandler {
	pub async fn handle(
		ctx: &SerenityContext,
		event: &Event<'_>,
		_framework_ctx: FrameworkContext<'_, crate::GlobalState, crate::Error>,
		global_state: &crate::GlobalState,
	) -> crate::Result<()> {
		trace!("Received event");

		if let Event::Ready { data_about_bot } = event {
			let user_tag = data_about_bot.user.tag();
			info!(global_state, "Connected to discord as {user_tag}.");

			ctx.set_activity(Activity::playing("kz_epiphany_v2"))
				.await;
		}

		Ok(())
	}
}
