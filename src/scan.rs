use std::fmt::{Display, Formatter, Result as FmtResult, Write as _};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Scan {
	first: String,
	second: String,
	matched: bool,
}

impl Scan {
	#[must_use]
	pub fn new(first: String, second: String) -> Self {
		let matched = first == second;
		Self {
			first,
			second,
			matched,
		}
	}

	#[must_use]
	pub fn first_scan(&self) -> &str {
		&self.first
	}

	#[must_use]
	pub fn second_scan(&self) -> &str {
		&self.second
	}

	#[must_use]
	pub const fn matched(&self) -> bool {
		self.matched
	}
}

impl Display for Scan {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str(self.first_scan())?;
		f.write_char('/')?;
		f.write_str(self.second_scan())
	}
}
