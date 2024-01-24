pub mod buffer;
mod eframe_error;
pub mod prelude;

pub use eframe::egui;
use eframe::egui::TextBuffer;

pub use self::eframe_error::EframeError;
use self::{
	buffer::Buffer,
	egui::{Color32, Layout, RichText},
};

#[derive(Debug)]
pub struct MainApp {
	first_buffer: Buffer,
	second_buffer: Buffer,
	status: Option<bool>,
}

impl MainApp {
	#[must_use]
	pub fn new() -> Self {
		Self {
			first_buffer: Buffer::new(14),
			second_buffer: Buffer::new(14),
			status: None,
		}
	}
}

impl Default for MainApp {
	fn default() -> Self {
		Self::new()
	}
}

impl eframe::App for MainApp {
	fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.heading("First scan");
			let first_scanner_res = ui.text_edit_singleline(&mut self.first_buffer);

			ui.heading("Second scan");

			let second_scanner_res = ui.text_edit_singleline(&mut self.second_buffer);

			let color = self
				.status
				.map(|status| if status { Color32::GREEN } else { Color32::RED });

			let window_frame = egui::Frame {
				fill: color.unwrap_or(ui.style().noninteractive().bg_fill),
				..egui::Frame::side_top_panel(ui.style())
			};

			if ui.button("Reset").clicked() {
				self.first_buffer.clear();
				self.second_buffer.clear();
				self.status = None;
			}

			egui::SidePanel::right("Status Window")
				.frame(window_frame)
				.show(ui.ctx(), |ui| {
					ui.with_layout(
						Layout::centered_and_justified(egui::Direction::TopDown),
						|ui| {
							if let Some(color) = color {
								ui.label(RichText::new("").color(color));
							} else {
								ui.spinner();
							}
						},
					);
				});

			if self.first_buffer.is_mutable() {
				first_scanner_res.request_focus();
			} else if self.second_buffer.is_mutable() {
				self.status = None;
				second_scanner_res.request_focus();
			} else {
				self.status = Some(self.first_buffer == self.second_buffer);
				self.first_buffer.clear();
				self.second_buffer.clear();
			}
		});
	}
}
