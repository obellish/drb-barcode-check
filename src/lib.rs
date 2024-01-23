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

#[derive(Debug)]
pub struct Scanner {
	pub device_id: Option<DeviceId>,
	buffer: String,
}

impl Scanner {
	#[must_use]
	pub const fn new() -> Self {
		Self {
			device_id: None,
			buffer: String::new(),
		}
	}

	#[must_use]
	pub const fn is_setup(&self) -> bool {
		self.device_id.is_some()
	}
}

impl Default for Scanner {
	fn default() -> Self {
		Self::new()
	}
}

impl Deref for Scanner {
	type Target = String;

	fn deref(&self) -> &Self::Target {
		&self.buffer
	}
}

impl DerefMut for Scanner {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.buffer
	}
}
