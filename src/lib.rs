
#![cfg_attr(feature = "internal_benches", allow(unstable_features), feature(test))]

#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(feature = "alloc")]
extern crate alloc;

pub mod g2;
pub mod g3;
pub mod g4;

pub mod sm2;
pub mod sm3;
pub mod sm4;

pub mod utils;
