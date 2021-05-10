#![doc(html_root_url = "https://docs.rs/TODO_CRATE_NAME/0.0.1")]
#![feature(lang_items)]
#![no_std]
#![warn(clippy::pedantic)]

use core::panic::PanicInfo;

#[cfg(doctest)]
pub mod readme {
	doc_comment::doctest!("../README.md");
}

#[no_mangle]
extern "C" fn setup() {}

#[no_mangle]
extern "C" fn r#loop() {}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
	loop {}
}
