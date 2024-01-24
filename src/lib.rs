use std::{
	error::Error as StdError,
	fmt::{Display, Formatter, Result as FmtResult},
	ops::{Deref, DerefMut},
};

use winit::{
	error::{EventLoopError, OsError},
	event::DeviceId,
};

#[derive(Debug)]
pub enum Error {
	EventLoop(EventLoopError),
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match self {
			Self::EventLoop(e) => Display::fmt(e, f),
		}
	}
}

impl From<EventLoopError> for Error {
	fn from(value: EventLoopError) -> Self {
		Self::EventLoop(value)
	}
}

impl From<OsError> for Error {
	fn from(value: OsError) -> Self {
		Self::EventLoop(value.into())
	}
}

impl StdError for Error {
	fn source(&self) -> Option<&(dyn StdError + 'static)> {
		match self {
			Self::EventLoop(e) => Some(e),
		}
	}
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
