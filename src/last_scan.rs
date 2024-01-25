use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LastScan {
	first_scan: String,
	second_scan: String,
	matched: bool,
}

impl LastScan {
	#[must_use]
	pub fn new(first: String, second: String) -> Self {
		let matched = first == second;
		Self {
			first_scan: first,
			second_scan: second,
			matched,
		}
	}
}
