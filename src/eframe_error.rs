use std::{
	error::Error,
	fmt::{Debug, Display, Formatter, Result as FmtResult},
};

use send_helper::SendHelper;

#[repr(transparent)]
pub struct EframeError(SendHelper<eframe::Error>);

impl Debug for EframeError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		Debug::fmt(&*self.0, f)
	}
}

impl Display for EframeError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		Display::fmt(&*self.0, f)
	}
}

impl Error for EframeError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		Some(&*self.0)
	}
}

impl From<eframe::Error> for EframeError {
	fn from(value: eframe::Error) -> Self {
		Self(SendHelper::new(value))
	}
}
