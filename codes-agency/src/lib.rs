/*!
This package provides a common code representing standards agencies.

The two core types, [Agency] and [Standard] work together to provide reporting
capabilities to other *codes* project packages. Specifically a package that
provides types corresponding to a standard definition can have an instance of
the [Standard] struct that describes the standard. This in turn references the
[Agency] that controls the standard.

```rust
use codes_agency::{Agency, Standard};

pub const ISO_4217: Standard = Standard::new_with_long_ref(
    Agency::ISO,
    "4217",
    "ISO 4217:2015",
    "Currency codes",
    "https://www.iso.org/iso-4217-currency-codes.html",
);
```
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

use std::{fmt::Display, str::FromStr};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This enumeration allows for the identification of well-known standards agencies. This is
/// useful in documenting crates that implement such standards.
///
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Agency {
    /// [Internet Assigned Numbers Authority](https://www.iana.org)
    IANA,
    /// [IEEE](https://www.ieee.org)
    IEEE,
    /// [The Internet Engineering Task Force](https://www.ietf.org)
    IETF,
    /// [International Organization for Standardization](https://www.iso.org)
    ISO,
}

///
/// This structure allows for the description of a specific standard, or specification,
/// issued by a well-known standards agency. Note that different versions of a standard
/// should be different instances with *at least* different long references.
///
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Standard {
    agency: Agency,
    short_ref: &'static str,
    long_ref: Option<&'static str>,
    title: &'static str,
    url: &'static str,
}

///
/// An error using in the `FromStr` implementation.
///
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Error {
    Unknown(String),
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
            "IANA" => Ok(Self::IANA),
            "IEEE" => Ok(Self::IEEE),
            "IETF" => Ok(Self::IETF),
            "ISO" => Ok(Self::ISO),
            _ => Err(Error::Unknown(s.to_string())),
        }
    }
}

impl Agency {
    ///
    /// Return the short name, usually an acronym or abbreviation, of the agency.
    /// This is usually the same set of characters as the variant name.
    ///
    pub const fn short_name(&self) -> &'static str {
        match self {
            Self::IANA => "IANA",
            Self::IEEE => "IEEE",
            Self::IETF => "IETF",
            Self::ISO => "ISO",
        }
    }

    ///
    /// A longer name, if one exists, for the agency.
    ///
    pub const fn name(&self) -> &'static str {
        match self {
            Self::IANA => "Internet Assigned Numbers Authority",
            Self::IEEE => "IEEE",
            Self::IETF => "The Internet Engineering Task Force",
            Self::ISO => "International Organization for Standardization",
        }
    }

    ///
    /// A URL for the agency.
    ///
    pub const fn url(&self) -> &'static str {
        match self {
            Self::IANA => "https://www.iana.org",
            Self::IEEE => "https://www.ieee.org",
            Self::IETF => "https://www.ietf.org",
            Self::ISO => "https://www.iso.org",
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Standard {
    ///
    /// Create a new Standard **without** a long reference value.
    ///
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

    ///
    /// Create a new Standard **with** a long reference value.
    ///
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

    ///
    /// Return the [Agency] that controls this standard.
    ///
    pub const fn agency(&self) -> Agency {
        self.agency
    }

    ///
    /// Return the short reference, or number, of this standard.
    ///
    pub const fn short_ref(&self) -> &'static str {
        self.short_ref
    }

    ///
    /// Return a longer reference, if one exists, for this standard,
    ///
    pub const fn long_ref(&self) -> Option<&&'static str> {
        self.long_ref.as_ref()
    }

    ///
    /// Return the textual title of this standard.
    ///
    pub const fn title(&self) -> &'static str {
        self.title
    }

    ///
    /// Return the URL for this standard.
    ///
    pub const fn url(&self) -> &'static str {
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
