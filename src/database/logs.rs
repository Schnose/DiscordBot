use crate::{error, logging::Level, yeet, Error, Result, State};
use sqlx::{types::chrono::NaiveDateTime, FromRow};

/// A log entry in the database
#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
pub struct LogRow {
	/// Autogenerated ID
	pub id: i64,
	/// `RUST_LOG` level
	pub level: i16,
	/// The log message
	pub content: String,
	/// When the log was created
	pub created_on: NaiveDateTime,
}

/// A log entry
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Log {
	/// Autogenerated ID
	pub id: u64,
	/// `RUST_LOG` level
	pub level: Level,
	/// The log message
	pub content: String,
	/// When the log was created
	pub created_on: NaiveDateTime,
}

impl Log {
	/// Parses a [`LogRow`] into a [`Log`].
	pub async fn from_row(
		LogRow { id, level, content, created_on }: LogRow,
		ctx: &crate::Context<'_>,
	) -> Result<Self> {
		let id = match u64::try_from(id) {
			Ok(id) => id,
			Err(err) => {
				error!(ctx.state(), "Invalid LogID found!\n\t{err:?}");
				yeet!(Error::InvalidLogID);
			}
		};

		let level = match level {
			0 => Level::Trace,
			1 => Level::Debug,
			2 => Level::Info,
			3 => Level::Warn,
			4 => Level::Error,
			level => {
				yeet!(Error::InvalidLogLevel(level));
			}
		};

		Ok(Self { id, level, content, created_on })
	}
}
