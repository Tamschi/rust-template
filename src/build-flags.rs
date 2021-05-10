use std::process::Command;

macro_rules! bprintln {
	($($tt:tt)*) => {
		eprintln!($($tt)*);
		println!($($tt)*);
	};
}

fn main() {
	eprintln!("Building Rust crate using Cargo...");
	let status = Command::new("cargo")
		.env("RUST_TARGET_PATH", std::env::current_dir().unwrap())
		.env("RUST_BACKTRACE", "1")
		.arg("build")
		.arg("--release")
		.args(&["--target", "avr-unknown-gnu-atmega328"])
		.args(&["-Z", "build-std=core"])
		.status()
		.expect("Failed to execute command `cargo build`.");
	assert!(status.success());

	bprintln!(
		"-L{}",
		dunce::canonicalize(
			std::env::current_dir()
				.unwrap()
				.join("target/avr-unknown-gnu-atmega328/release")
				.canonicalize()
				.unwrap()
		)
		.unwrap()
		.display()
		.to_string()
		.replace('\\', "/")
	);
	bprintln!("-lTODO_CRATE_NAME");
}
