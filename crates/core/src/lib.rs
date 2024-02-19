#![deny(warnings)]

#[macro_use]
extern crate lazy_static;

pub mod engine;
pub mod utils;

mod config;
mod test;

#[cfg(target_arch = "wasm32")]
mod wasm;
