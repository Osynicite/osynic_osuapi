#[cfg(feature = "wasm")]
pub mod gloo;
#[cfg(feature = "not-wasm")]
pub mod request;