#![allow(unused)]

use std::sync::atomic::{AtomicBool, Ordering};

use drb_barcode_check::Result;
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
	let _window = WindowBuilder::new().build(&event_loop)?;

	let mut first_buffer = String::new();
	let mut second_buffer = String::new();

	let mut text = String::new();

	event_loop.run(move |event, elwt| {
		if let Event::WindowEvent { event, .. } = event {
			match event {
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
							if first_buffer.is_empty() {
								first_buffer.push_str(&text);
							} else if second_buffer.is_empty() {
								second_buffer.push_str(&text);
							}
							text.clear();
						}

						if first_buffer.len() == 14 && second_buffer.len() == 14 {
							first_buffer.clear();
							second_buffer.clear();
						}
					}
				}
				_ => std::thread::yield_now(),
			}
		}
	})?;

	Ok(())
}
