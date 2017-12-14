//! ICMP protocol support and implementations.
//!
//! This package is useful for sending and receiving packets
//! over the Internet Control Message Protocol (ICMP). It
//! currently offers a simple API and implementation for `ping`.
//!
//! ## Installation
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! libicmp = "0.1.1"
//! ```
//!
//! ## Examples
//!
//! ```
//! use libicmp::PingBuilder;
//!
//! let p = PingBuilder::new()
//!     .host("127.0.0.1")
//!     .num_pings(5)
//!     .interval_secs(1)
//!     .timeout_secs(5)
//!     .debug(true)
//!     .build();
//! p.ping();
//! ```
extern crate nix;
extern crate libc;
extern crate rand;
extern crate byteorder;

pub mod packet;
pub mod socket;
pub mod icmp;
pub mod ping;

pub use icmp::Icmp;
pub use ping::PingBuilder;
