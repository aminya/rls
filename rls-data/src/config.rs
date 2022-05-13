#[cfg(feature = "derive")]
use serde::{Deserialize, Serialize};

/// Used to configure save-analysis.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "derive", derive(Serialize, Deserialize))]
pub struct Config {
    /// File to output save-analysis data to.
    pub output_file: Option<String>,
    /// Include all documentation for items. (If `false`, only includes the
    /// summary (first paragraph) for each item).
    pub full_docs: bool,
    /// If true only includes data for public items in a crate (useful for
    /// library crates).
    pub pub_only: bool,
    /// If true only includes data for items reachable from the crate root.
    pub reachable_only: bool,
    /// True if and only if the analysed crate is part of the standard Rust distro.
    pub distro_crate: bool,
    /// Include signature information.
    pub signatures: bool,
    /// Include experimental borrow data.
    pub borrow_data: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            output_file: None,
            full_docs: true,
            pub_only: false,
            reachable_only: false,
            distro_crate: false,
            signatures: true,
            borrow_data: true,
        }
    }
}
