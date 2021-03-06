#![no_std]
#![feature(alloc)]
#![feature(nll)]

extern crate alloc;
#[macro_use]
extern crate log;
extern crate spin;

// To use `println!` in test
#[cfg(test)]
#[macro_use]
extern crate std;

pub mod paging;
pub mod cow;
pub mod swap;
pub mod memory_set;
mod addr;

pub use addr::*;