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

use codes_agency::{Agency, Code, Standard};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub const ISO_4217: Standard = Standard::new_with_long_ref(
    Agency::ISO,
    "4217",
    "ISO 4217:2015",
    "Currency codes",
    "https://www.iso.org/iso-4217-currency-codes.html",
);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde_derive", derive(Deserialize, Serialize))]
pub enum Currency {
    USD,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde_derive", derive(Deserialize, Serialize))]
pub enum Error {
    Unknown(String),
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.alpha_code())
    }
}

impl FromStr for Currency {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "USD" => Ok(Self::USD),
            _ => Err(Error::Unknown(s.to_string())),
        }
    }
}

impl Code for Currency {
    fn standard() -> &'static Standard
    where
        Self: Sized,
    {
        &ISO_4217
    }
}

impl Currency {
    pub fn alpha_code(&self) -> &'static str {
        match self {
            Self::USD => "USD",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::USD => "",
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Unknown(s) =>
                    format!("The code {:?} is not a known currency from ISO-4166", s),
            }
        )
    }
}

impl std::error::Error for Error {}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
