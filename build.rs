#![allow(clippy::unnecessary_wraps, unused)]

use std::{
	env, fs,
	path::{Path, PathBuf},
	time::Duration,
};

use build_helper::Result;

const ROOT: &str = env!("CARGO_MANIFEST_DIR");
const GENERATOR: &str = "Visual Studio 16 2019";

fn main() -> Result<()> {
	println!("cargo:rerun-if-changed=NULL");
	let lib_dir = Path::new(ROOT).join("lib");
	let msgpack_dir = lib_dir.join("msgpack11");
	let sick_scan_dir = lib_dir.join("sick_scan_xd");

	let _ = fs::remove_dir(sick_scan_dir.join("build"));

	let init_msgpack_build_dir = build_msgpack(msgpack_dir)?;

	build_sick_scan_xd(sick_scan_dir)?;

	Ok(())
}

fn build_msgpack<P>(in_dir: P) -> Result<PathBuf>
where
	P: AsRef<Path>,
{
	Ok(cmake::Config::new(in_dir)
		.define("MSGPACK11_BUILD_TESTS", "0")
		.generator(GENERATOR)
		.no_build_target(true)
		.build())
}

fn build_sick_scan_xd<P>(in_dir: P) -> Result<PathBuf>
where
	P: AsRef<Path>,
{
	let _ = fs::create_dir(in_dir.as_ref().join("build"));
	Ok(cmake::Config::new(in_dir)
		.define("ROS_VERSION", "0")
		.env("_os", "x64")
		.env("_msvc", "Visual Studio 2019")
		.generator(GENERATOR)
		.build())
}
