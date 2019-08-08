#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate grin_core;
extern crate grin_p2p;

use grin_core::ser;
use grin_p2p::msg::Shake;

fuzz_target!(|data: &[u8]| {
	let mut d = data.clone();
	let _t: Result<Shake, ser::Error> = ser::deserialize(&mut d);
});
