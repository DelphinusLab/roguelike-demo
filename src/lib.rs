#![deny(warnings)]

#[macro_use]
extern crate lazy_static;

mod config;
pub mod engine;
mod test;

#[cfg(target_arch = "wasm32")]
mod wasm;
