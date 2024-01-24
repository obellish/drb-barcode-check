#![allow(unused)]

use std::sync::atomic::{AtomicBool, Ordering};

use drb_barcode_check::{Result, Scanner};
use winit::{
	event::{ElementState, Event, WindowEvent},
	event_loop::{EventLoop, EventLoopBuilder},
	keyboard::{Key, NamedKey},
	platform::{modifier_supplement::KeyEventExtModifierSupplement, windows::DeviceIdExtWindows},
	window::WindowBuilder,
};

const CONTROL_CODE: &str = "-CONTROL CODE-";

fn main() -> Result<()> {
	let event_loop = EventLoop::new()?;
	let window = WindowBuilder::new().build(&event_loop)?;

	let mut first_scanner = Scanner::new();
	let mut second_scanner = Scanner::new();

	let mut text = String::new();

	event_loop.run(move |event, elwt| match event {
		Event::WindowEvent { event, .. } => match event {
			WindowEvent::CloseRequested => elwt.exit(),
			WindowEvent::KeyboardInput {
				event, device_id, ..
			} => {
				if event.state == ElementState::Pressed && !event.repeat {
					match event.logical_key {
						Key::Character(c) => text.push_str(c.as_str()),
						Key::Named(named) => {
							if let Some(t) = named.to_text() {
								text.push_str(t);
							}
						}
						_ => {}
					}
					if text.len() == 14 {
						println!("{device_id:?}");
						match (first_scanner.is_setup(), second_scanner.is_setup()) {
							(false, _) if text == CONTROL_CODE => {
								first_scanner.device_id = Some(device_id);
								println!("first scanner setup complete");
							}
							(true, false) if text == CONTROL_CODE => {
								second_scanner.device_id = Some(device_id);

								assert_ne!(
									first_scanner.device_id, second_scanner.device_id,
									"same scanner was used twice during setup"
								);
								println!("second scanner setup complete");
								println!("setup complete");
							}
							(true, true) => {
								if first_scanner.device_id == Some(device_id) {
									first_scanner.push_str(&text);
								} else {
									second_scanner.push_str(&text);
								}
							}
							_ => {}
						}
						text.clear();
					}
				}
			}
			_ => {}
		},
		Event::DeviceEvent { device_id, event } => {
			dbg!(device_id);
			dbg!(event);
		}
		_ => {}
	})?;

	Ok(())
}
