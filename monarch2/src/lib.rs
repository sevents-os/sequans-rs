#![cfg_attr(not(test), no_std)]

//! # monarch2
//!
//! This crate supports chips from the Sequans [Monarch 2](https://sequans.com/products/monarch-2/)
//! LTE Platform family using AT commands based interface.
//! It can be used both on `no_std` and `std` platforms.

// This mod MUST go first, so that the others see its macros.
pub(crate) mod fmt;

pub mod command;
pub mod error;
pub mod modem;
