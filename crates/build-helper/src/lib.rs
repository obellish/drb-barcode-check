use std::{
	env::VarError,
	error::Error,
	fmt::{Display, Formatter, Result as FmtResult},
	io::Error as IoError,
};

#[derive(Debug)]
pub enum BuildError {
	Io(IoError),
	Env(VarError),
}

impl Display for BuildError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match self {
			Self::Io(e) => Display::fmt(e, f),
			Self::Env(e) => Display::fmt(e, f),
		}
	}
}

impl Error for BuildError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			Self::Io(e) => Some(e),
			Self::Env(e) => Some(e),
		}
	}
}

impl From<IoError> for BuildError {
	fn from(value: IoError) -> Self {
		Self::Io(value)
	}
}

impl From<VarError> for BuildError {
	fn from(value: VarError) -> Self {
		Self::Env(value)
	}
}

pub type Result<T, E = BuildError> = std::result::Result<T, E>;
