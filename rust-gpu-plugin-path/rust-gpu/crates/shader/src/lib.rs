#![no_std]
#![feature(asm_experimental_arch)]

pub use bevy_pbr_rust::prelude::*;

#[path = "case1-dynamic-rust.rs"]
mod case1;

#[path = "case2-gradient-rust.rs"]
mod case2;

pub use case1::*;
pub use case2::*;
