//! # boxcars (also written boxca-rs)
//!
//! boxcars is an example of a [Rocket League](http://www.rocketleaguegame.com/) replay parser
//! written in Rust using [nom](https://github.com/Geal/nom) for parsing and
//! [serde](https://github.com/serde-rs/serde) for serialization. Emphasis on example, as this
//! library in no way competes with the other feature complete parsers such as
//! [Octane](https://github.com/tfausak/octane) and
//! [`RocketLeagueReplayParser`](https://github.com/jjbott/RocketLeagueReplayParser). Rather, let
//! boxcars be a good example of Rust code using nom, and serde as extensive examples are hard to
//! come by. While lacking feature completeness and user friendly error message -- [among other
//! issues](https://github.com/nickbabcock/boxcars/issues), tests and documentation strive to be
//! thorough.
//!
//! ```
//! extern crate boxcars;
//! extern crate nom;
//! extern crate serde_json;
//!
//! use std::fs::File;
//! use std::io::Read;
//!
//! # let filename = "assets/rumble.replay";
//! let mut f = File::open(filename).unwrap();
//! let mut buffer = vec![];
//! f.read_to_end(&mut buffer).unwrap();
//! let b = boxcars::parse(&buffer, true);
//!
//! match b {
//!     Ok(val) => {
//!         let serialized = serde_json::to_string(&val).unwrap();
//!         println!("{}", serialized);
//!     }
//!     _ => {
//!         println!("Oh no we failed to parse");
//!     }
//! }
//! ```

#![cfg_attr(feature = "nightly", feature(test))]

extern crate encoding;
#[macro_use]
extern crate nom;
extern crate serde;

#[macro_use]
extern crate error_chain;

#[cfg(feature = "nightly")]
extern crate test;

#[cfg(test)]
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

pub use self::models::*;
pub use self::parsing::*;
pub use self::errors::*;
mod parsing;
mod models;
mod crc;
mod errors;
