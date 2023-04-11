use {
	crate::{
		error::{Error, Result},
		state::State,
	},
	poise::{
		serenity_prelude::{Activity, Context},
		Event, FrameworkContext,
	},
	tracing::{info, trace},
};

// &Context, &Event, FrameworkContext<State, Error>, &State
pub async fn handle(
	ctx: &Context,
	event: &Event<'_>,
	_framework_ctx: FrameworkContext<'_, State, Error>,
	_state: &State,
) -> Result<()> {
	match event {
		Event::Ready { data_about_bot } => {
			info!("Connected to Discord as {}!", data_about_bot.user.tag());

			ctx.set_activity(Activity::playing("kz_epiphany_v2"))
				.await;
		}
		event => trace!("Received event: {event:?}"),
	};

	Ok(())
}
