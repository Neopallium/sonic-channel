//! # Sonic Channel
//! Rust client for [sonic] search backend.
//!
//!
//! ## Example usage
//!
//! ### Search channel
//!
//! Note: This example requires enabling the `search` feature, enabled by default.
//!
//! ```rust,no_run
//! use sonic_channel::*;
//!
//! fn main() -> result::Result<()> {
//!     let channel = SearchChannel::start(
//!         "localhost:1491",
//!         "SecretPassword",
//!     )?;
//!
//!     let objects = channel.query("collection", "bucket", "recipe")?;
//!     log::debug!("{}", objects);
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Ingest channel
//!
//! Note: This example requires enabling the `ingest` feature.
//!
//! ```rust,no_run
//! use sonic_channel::*;
//!
//! fn main() -> result::Result<()> {
//!     let mut channel = IngestChannel::start(
//!         "localhost:1491",
//!         "SecretPassword",
//!     )?;
//!
//!     let pushed = channel.push("collection", "bucket", "object:1", "my best recipe")?;
//!     // or
//!     // let pushed = channel.push_with_locale("collection", "bucket", "object:1", "Мой лучший рецепт", "rus")?;
//!     log::debug!("{}", pushed);
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Control channel
//!
//! Note: This example requires enabling the `control` feature.
//!
//! ```rust,no_run
//! use sonic_channel::*;
//!
//! fn main() -> result::Result<()> {
//!     let mut channel = ControlChannel::start(
//!         "localhost:1491",
//!         "SecretPassword",
//!     )?;
//!
//!     let result = channel.consolidate()?;
//!     assert_eq!(result, true);
//!
//!     Ok(())
//! }
//! ```
//!
//! [sonic]: https://github.com/valeriansaliou/sonic

// Rustc lints.
#![deny(
    missing_debug_implementations,
    unsafe_code,
    unstable_features,
    unused_imports,
    unused_qualifications
)]
#![warn(missing_docs)]
// Clippy lints
#![deny(clippy::all)]

#[cfg(not(any(feature = "ingest", feature = "search", feature = "control")))]
compile_error!(
    r#"Either features "ingest" or "search" or "control" must be enabled for "sonic-channel" crate"#
);

#[macro_use]
mod macroses;

mod channels;
mod commands;

/// Contains sonic channel error type and custom Result type for easy configure your functions.
pub mod result;

pub use channels::*;

#[macro_use]
extern crate lazy_static;
extern crate regex;

#[cfg(test)]
mod tests {
    use crate::channels::ChannelMode;

    #[test]
    fn format_channel_enums() {
        assert_eq!(format!("{}", ChannelMode::Search), String::from("search"));
        assert_eq!(format!("{}", ChannelMode::Ingest), String::from("ingest"));
        assert_eq!(format!("{}", ChannelMode::Control), String::from("control"));
    }

    //TODO: write tests with sonic server
}
