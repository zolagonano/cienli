//! An Implementation of historical ciphers.
//!
//! **Note:** If you only need to use one or a couple of ciphers, you can disable default features and specify those features you need.
//! For example, if you only need to use caesar cipher in your crate, you can have this in your Cargo.toml:
//!
//! ```toml
//! [dependencies]
//! cienli = { version = "0.3.1", default-features = false, features = ["caesar"]}
//! ```
//!
pub mod ciphers;
pub mod common;
