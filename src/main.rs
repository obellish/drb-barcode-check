#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

use drb_barcode_check::{prelude::*, EframeError, MainApp};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

fn main() -> Result<()> {
	let log_filter_layer = EnvFilter::try_from_default_env()
		.or_else(|_| EnvFilter::try_new("debug"))
		.into_diagnostic()?;

	// We don't need thread IDs/names because it's single threaded.
	let log_fmt_layer = fmt::layer()
		.pretty()
		.with_thread_ids(false)
		.with_thread_names(false);

	tracing_subscriber::registry()
		.with(log_filter_layer)
		.with(log_fmt_layer)
		.try_init()
		.into_diagnostic()?;

	eframe::run_native(
		"DRB Barcode Checker",
		eframe::NativeOptions::default(),
		Box::new(|cc| Box::new(MainApp::new(cc))),
	)
	.map_err(EframeError::from)
	.into_diagnostic()?;

	Ok(())
}
