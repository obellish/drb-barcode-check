use std::ops::Deref;

use super::egui::TextBuffer;

/// A buffer with a set capacity that it will not go over.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Buffer {
	inner: String,
	cap: usize,
}

impl Buffer {
	#[must_use]
	pub fn new(cap: usize) -> Self {
		Self {
			inner: String::with_capacity(cap),
			cap,
		}
	}

	fn cut_to_capacity(&mut self) -> usize {
		// let mut count = 0;
		// while self.inner.len() > self.cap {
		// 	self.inner.pop();
		// 	count += 1;
		// }

		// count
		let old_length = self.inner.len();

		self.inner.truncate(self.cap);

		old_length - self.inner.len()
	}

	pub fn clear(&mut self) {
		self.inner.clear();
	}
}

impl Deref for Buffer {
	type Target = str;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl TextBuffer for Buffer {
	fn is_mutable(&self) -> bool {
		self.inner.len() < self.cap
	}

	fn as_str(&self) -> &str {
		self
	}

	fn insert_text(&mut self, text: &str, char_index: usize) -> usize {
		let inserted = self.inner.insert_text(text, char_index);

		let removed = self.cut_to_capacity();

		inserted - removed
	}

	fn delete_char_range(&mut self, char_range: std::ops::Range<usize>) {
		self.inner.delete_char_range(char_range);
	}

	fn clear(&mut self) {
		self.inner.clear();
	}

	fn take(&mut self) -> String {
		std::mem::take(&mut self.inner)
	}

	fn replace_with(&mut self, text: &str) {
		self.inner = text.to_owned();
		self.cut_to_capacity();
	}
}

#[cfg(test)]
mod tests {
	use super::Buffer;

	#[test]
	fn trims_correctly() {
		let mut buf = Buffer::new(5);

		buf.inner.push_str("Hello, world!");
		let cut = buf.cut_to_capacity();

		assert_eq!(buf.inner, "Hello");
		assert_eq!(cut, 8);
	}
}
