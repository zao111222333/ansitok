#![recursion_limit = "256"]
#![cfg_attr(not(any(feature = "std", test)), no_std)]
#![warn(missing_docs)]

//! This is a crate is made for parsing ANSI escape sequences.
//!
//! The list of covered sequences.
//!
//! * Cursor Position
//! * Cursor {Up, Down, Forward, Backward}
//! * Cursor {Save, Restore}
//! * Erase Display
//! * Erase Line
//! * Set Graphics mode
//! * Set/Reset Text Mode
//!
//! # Usage
//!
//! ```
//! use ansitok::{parse_ansi, Output};
//!
//! let text = "\x1b[31;1;4mHello World\x1b[0m";
//!
//! for output in parse_ansi(text) {
//!     match output {
//!         Output::Text(text) => println!("Got a text: {:?}", text),
//!         Output::Escape(esc) => println!("Got an escape sequence: {:?}", esc),
//!     }
//! }
//! ```
//!
//! Parse SGR.
//!
//! ```
//! use ansitok::{parse_ansi, parse_ansi_sgr, AnsiSequence, Output};
//!
//! let text = "\x1b[31;1;4mHello World\x1b[0m \x1b[38;2;255;255;0m!!!\x1b[0m";
//!
//! for output in parse_ansi(text) {
//!     if let Output::Escape(AnsiSequence::SelectGraphicRendition(sgr)) = output {
//!         for style in parse_ansi_sgr(sgr) {
//!             println!("{:?}", style);
//!         }
//!     }
//! }
//! ```

mod parse;
mod parsers;

pub use parse::{AnsiColor, AnsiSequence, VisualAttribute};
pub use parsers::{parse_ansi, parse_ansi_sgr, AnsiSequenceParser, Output, SGRParser};
