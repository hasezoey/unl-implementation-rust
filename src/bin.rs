#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]

mod lib;

/// Hello
pub fn main() {
	println!("hi {}", lib::libfn());
}
