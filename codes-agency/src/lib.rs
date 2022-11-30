/*!
One-line description.

More detailed description, with

# Example

YYYYY

# Features

*/

#![warn(
    unknown_lints,
    // ---------- Stylistic
    absolute_paths_not_starting_with_crate,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    macro_use_extern_crate,
    nonstandard_style, /* group */
    noop_method_call,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    // ---------- Future
    future_incompatible, /* group */
    rust_2021_compatibility, /* group */
    // ---------- Public
    missing_debug_implementations,
    // missing_docs,
    unreachable_pub,
    // ---------- Unsafe
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    // ---------- Unused
    unused, /* group */
)]
#![deny(
    // ---------- Public
    exported_private_dependencies,
    private_in_public,
    // ---------- Deprecated
    anonymous_parameters,
    bare_trait_objects,
    ellipsis_inclusive_range_patterns,
    // ---------- Unsafe
    deref_nullptr,
    drop_bounds,
    dyn_drop,
)]

use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde_derive", derive(Deserialize, Serialize))]
pub enum Agency {
    ISO,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde_derive", derive(Deserialize, Serialize))]
pub struct Standard {
    agency: Agency,
    short_ref: &'static str,
    long_ref: Option<&'static str>,
    title: &'static str,
    #[cfg(not(feature = "no-standard-url"))]
    url: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde_derive", derive(Deserialize, Serialize))]
pub enum Error {
    Unknown(String),
}

pub trait Code: Display + FromStr {
    fn standard() -> &'static Standard
    where
        Self: Sized;
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for Agency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.short_name())
    }
}

impl FromStr for Agency {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ISO" => Ok(Self::ISO),
            _ => Err(Error::Unknown(s.to_string())),
        }
    }
}

impl Agency {
    pub fn short_name(&self) -> &'static str {
        match self {
            Self::ISO => "ISO",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::ISO => "International Organization for Standardization",
        }
    }

    #[cfg(not(feature = "no-agency-url"))]
    pub fn url(&self) -> &'static str {
        match self {
            Self::ISO => "https://www.iso.org/",
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Standard {
    #[cfg(not(feature = "no-standard-url"))]
    pub const fn new(
        agency: Agency,
        short_ref: &'static str,
        title: &'static str,
        url: &'static str,
    ) -> Self {
        Self {
            agency,
            short_ref,
            long_ref: None,
            title,
            url,
        }
    }

    #[cfg(not(feature = "no-standard-url"))]
    pub const fn new_with_long_ref(
        agency: Agency,
        short_ref: &'static str,
        long_ref: &'static str,
        title: &'static str,
        url: &'static str,
    ) -> Self {
        Self {
            agency,
            short_ref,
            long_ref: Some(long_ref),
            title,
            url,
        }
    }

    #[cfg(feature = "no-standard-url")]
    pub const fn new(agency: Agency, short_ref: &'static str, title: &'static str) -> Self {
        Self {
            agency,
            short_ref,
            long_ref: None,
            title,
        }
    }

    #[cfg(feature = "no-standard-url")]
    pub const fn new_with_long_ref(
        agency: Agency,
        short_ref: &'static str,
        long_ref: &'static str,
        title: &'static str,
    ) -> Self {
        Self {
            agency,
            short_ref,
            long_ref: Some(long_ref),
            title,
        }
    }

    pub fn agency(&self) -> Agency {
        self.agency
    }

    pub fn short_ref(&self) -> &'static str {
        self.short_ref
    }

    pub fn long_ref(&self) -> Option<&&'static str> {
        self.long_ref.as_ref()
    }

    pub fn title(&self) -> &'static str {
        self.title
    }

    #[cfg(not(feature = "no-standard-url"))]
    pub fn url(&self) -> &'static str {
        self.url
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Unknown(s) => format!("The short name {:?} is not a known agency", s),
            }
        )
    }
}

impl std::error::Error for Error {}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
