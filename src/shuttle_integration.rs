use {
	crate::{error::Error, state::State},
	poise::{async_trait, FrameworkBuilder},
	std::net::SocketAddr,
};

pub type ShuttleResult = Result<SchnoseBot, shuttle_service::Error>;

pub struct SchnoseBot {
	pub framework: FrameworkBuilder<State, Error>,
}

impl SchnoseBot {
	pub fn new(framework: FrameworkBuilder<State, Error>) -> Self {
		Self { framework }
	}
}

#[async_trait]
impl shuttle_service::Service for SchnoseBot {
	async fn bind(self, _: SocketAddr) -> Result<(), shuttle_service::Error> {
		self.framework
			.run()
			.await
			.expect("Failed to run SchnoseBot.");

		Ok(())
	}
}
