/*
 * rustup update
 * rustup target add wasm32-unknown-unknown --toolchain nightly
 * rustc +nightly --target wasm32-unknown-unknown -O --crate-type=cdylib add1.rs
 */
#[no_mangle]
pub fn add_one(x: i32) -> i32 {
  x + 1
}
