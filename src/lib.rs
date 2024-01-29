pub mod buffer;
mod eframe_error;
pub mod prelude;
pub mod scan;

pub use eframe::egui;
use eframe::{egui::TextBuffer, CreationContext};

pub use self::eframe_error::EframeError;
use self::{buffer::Buffer, scan::Scan};

#[derive(Debug)]
pub struct MainApp {
	first_buffer: Buffer,
	second_buffer: Buffer,
	scans: [Option<Scan>; 21],
}

impl MainApp {
	#[must_use]
	pub fn new(cc: &CreationContext<'_>) -> Self {
		egui_extras::install_image_loaders(&cc.egui_ctx);
		Self {
			first_buffer: Buffer::new(14),
			second_buffer: Buffer::new(14),
			scans: default_array(),
		}
	}

	fn reset(&mut self) {
		self.first_buffer.clear();
		self.second_buffer.clear();
		self.scans = default_array();
	}
}

impl eframe::App for MainApp {
	fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.heading("First scan");
			let first_scanner_res = ui.text_edit_singleline(&mut self.first_buffer);

			ui.heading("Second scan");

			let second_scanner_res = ui.text_edit_singleline(&mut self.second_buffer);

			if ui.button("Reset").clicked() {
				self.reset();
			}

			egui::SidePanel::right("Last 20 Scans")
				.resizable(true)
				.min_width(201.0)
				.show(ui.ctx(), |ui| {
					egui_extras::StripBuilder::new(ui)
						.sizes(egui_extras::Size::remainder(), self.scans.len() - 1)
						.vertical(|mut strip| {
							for (i, scan) in self.scans.iter().enumerate() {
								if i == self.scans.len() - 1 {
								} else if let Some(scan) = scan {
									strip.cell(|ui| {
										egui::Frame::none()
											.fill(if scan.matched() {
												egui::Color32::GREEN
											} else {
												egui::Color32::RED
											})
											.stroke((1.0, egui::Color32::GRAY))
											.show(ui, |ui| {
												ui.label(
													egui::RichText::new(scan.to_string())
														.color(egui::Color32::BLACK),
												);
											});
									});
								} else {
									strip.empty();
								}
							}
						});
				});

			if self.first_buffer.is_mutable() {
				first_scanner_res.request_focus();
				self.second_buffer.clear();
			} else if self.second_buffer.is_mutable() {
				second_scanner_res.request_focus();
			} else {
				self.scans[self.scans.len() - 1] = Some(Scan::new(
					self.first_buffer.take(),
					self.second_buffer.take(),
				));
				self.scans.rotate_left(1);
				self.scans.last_mut().take();
			}
		});
	}
}

fn default_array<T, const N: usize>() -> [Option<T>; N] {
	std::array::from_fn(|_| None)
}
