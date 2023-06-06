use crate::{yeet, Error, Result};
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(missing_docs)]
pub enum Level {
	Trace,
	Debug,
	Info,
	Warn,
	Error,
}

impl Display for Level {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(match self {
			Level::Trace => "TRACE",
			Level::Debug => "DEBUG",
			Level::Info => "INFO",
			Level::Warn => "WARN",
			Level::Error => "ERROR",
		})
	}
}

impl From<Level> for (u8, u8, u8) {
	fn from(level: Level) -> Self {
		match level {
			Level::Trace => (148, 226, 213),
			Level::Debug => (137, 180, 250),
			Level::Info => (166, 227, 161),
			Level::Warn => (249, 226, 175),
			Level::Error => (243, 139, 168),
		}
	}
}

impl TryFrom<u8> for Level {
	type Error = Error;

	fn try_from(level: u8) -> Result<Self> {
		Ok(match level {
			0 => Self::Trace,
			1 => Self::Debug,
			2 => Self::Info,
			3 => Self::Warn,
			4 => Self::Error,
			level => {
				yeet!("`{}` is an invalid log level.", level);
			}
		})
	}
}

/// Logging macro that will format the input as a string and save it to a database. It will also
/// send the content as a message in a discord channel.
#[macro_export]
macro_rules! log {
	( $level:literal, $state:expr, $( $args:tt )* ) => {
		'log: {
			let content = format!( $( $args )* );

			/* First save the logs to the database. */
			let mut query = ::sqlx::QueryBuilder::new("INSERT INTO ");

			query
				.push(&$state.logs_table)
				.push(" (level, content) ");

			query.push_values([($level, &content)], |mut query, (level, content)| {
				query
					.push_bind(level)
					.push_bind(content);
			});

			if let Err(err) = query.build().execute(&$state.database_pool).await {
				::tracing::error!("Failed to save logs to database.\n\t{err:?}");
				break 'log;
			}

			/* Now send logs to STDOUT/STDERR via tracing */

			let Ok(level) = $crate::logging::Level::try_from($level) else {
				::tracing::error!("Got invalid log level `{}`.", $level);
				break 'log;
			};

			match level {
				$crate::logging::Level::Trace => {
					::tracing::trace!("{content}");
				}
				$crate::logging::Level::Debug => {
					::tracing::debug!("{content}");
				}
				$crate::logging::Level::Info => {
					::tracing::info!("{content}");
				}
				$crate::logging::Level::Warn => {
					::tracing::warn!("{content}");
				}
				$crate::logging::Level::Error => {
					::tracing::error!("{content}");
				}
			};

			/* And finally, send the logs to discord */

			let Some(ref http) = $state.http else {
				// State has not been fully initialized yet.
				break 'log;
			};

			let date = ::sqlx::types::chrono::Utc::now().format("[%d-%m-%Y | %H:%M:%S]");

			if let Err(err) = $state.logs_channel.send_message(http, |message| {
				let color: (u8, u8, u8) = level.into();

				// TODO: Log arguments to command?
				// I don't know where I could get that info, but it would be nice
				// to include.
				message.embed(|embed| {
					embed.color(color)
						.title(format!("[{level}]"))
						.description(content)
						.footer(|f| f.text(date.to_string()))
				})

			}).await {
				::tracing::error!("Failed to send logs to discord.\n\t{err:?}");
				break 'log;
			}
		}
	};
}

/// TRACE-level Logging macro that will format the input as a string and save it to a database. It
/// will also send the content as a message in a discord channel.
#[macro_export]
macro_rules! trace {
	( $ctx:expr, $( $args:tt )* ) => { $crate::log!(0, $ctx, $($args)*) };
	( $( $args:tt )* ) => { ::tracing::trace!( $( $args )* ) };
}

/// DEBUG-level Logging macro that will format the input as a string and save it to a database. It
/// will also send the content as a message in a discord channel.
#[macro_export]
macro_rules! debug {
	( $ctx:expr, $( $args:tt )* ) => { $crate::log!(1, $ctx, $($args)*) };
	( $( $args:tt )* ) => { ::tracing::debug!( $( $args )* ) };
}

/// INFO-level Logging macro that will format the input as a string and save it to a database. It
/// will also send the content as a message in a discord channel.
#[macro_export]
macro_rules! info {
	( $ctx:expr, $( $args:tt )* ) => { $crate::log!(2, $ctx, $($args)*) };
	( $( $args:tt )* ) => { ::tracing::info!( $( $args )* ) };
}

/// WARN-level Logging macro that will format the input as a string and save it to a database. It
/// will also send the content as a message in a discord channel.
#[macro_export]
macro_rules! warn {
	( $ctx:expr, $( $args:tt )* ) => { $crate::log!(3, $ctx, $($args)*) };
	( $( $args:tt )* ) => { ::tracing::warn!( $( $args )* ) };
}

/// ERROR-level Logging macro that will format the input as a string and save it to a database. It
/// will also send the content as a message in a discord channel.
#[macro_export]
macro_rules! error {
	($ctx:expr, $( $args:tt )* ) => { $crate::log!(4, $ctx, $($args)*) };
	( $( $args:tt )* ) => { ::tracing::error!( $( $args )* ) };
}
