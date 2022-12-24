/*!
Support capabilities for `codes-*` packages.

# Features

* `build` - Tools for build scripts.
* `csv_tools` - Tools for reading and processing CSV files, requires `build`.

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

use std::{
    fmt::{Debug, Display},
    hash::Hash,
    str::FromStr,
};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait Code<T>: Clone + Debug + Display + FromStr + Into<T> + PartialEq + Eq + Hash {
    fn is_valid<S>(s: S) -> bool
    where
        S: AsRef<str>,
    {
        Self::from_str(s.as_ref()).is_ok()
    }
}

pub trait FixedLengthCode {
    fn fixed_length() -> usize;
}

pub trait VariableLengthCode {
    fn min_length() -> usize;

    fn max_length() -> usize;
}

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

#[macro_export]
macro_rules! code_as_str {
    ($type_name:ty, $inner:ident) => {
        impl $type_name {
            fn as_str(&self) -> &str {
                &self.$inner
            }
        }
    };
    ($type_name:ty) => {
        impl $type_name {
            fn as_str(&self) -> &str {
                &self.0
            }
        }
    };
}

#[macro_export]
macro_rules! code_impl {
    ($type_name:ty, $id_field:ident, $ltime:lifetime $id_type_ref:ty, $id_type:ty, $from_fn:ident) => {
        impl ::std::fmt::Display for $type_name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}", self.as_ref())
            }
        }

        impl ::std::convert::AsRef<$id_type_ref> for $type_name {
            fn as_ref(&self) -> &$ltime $id_type_ref {
                &self.$id_field()
            }
        }

        impl ::std::ops::Deref for $type_name {
            type Target = $id_type_ref;

            fn deref(&self) -> &$ltime Self::Target {
                &self.$id_field()
            }
        }

        impl ::std::convert::From<$type_name> for $id_type {
            fn from(v: $type_name) -> Self {
                v.$id_field().$from_fn()
            }
        }

        impl $crate::Code<$id_type> for $type_name {}
    };
    ($type_name:ty, $id_field:ident, $id_type_ref:ty, $id_type:ty, $from_fn:ident) => {
        impl ::std::fmt::Display for $type_name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}", self.as_ref())
            }
        }

        impl ::std::convert::AsRef<$id_type_ref> for $type_name {
            fn as_ref(&self) -> &$id_type_ref {
                &self.$id_field()
            }
        }

        impl ::std::ops::Deref for $type_name {
            type Target = $id_type_ref;

            fn deref(&self) -> &Self::Target {
                &self.$id_field()
            }
        }

        impl ::std::convert::From<$type_name> for $id_type {
            fn from(v: $type_name) -> Self {
                v.$id_field().$from_fn()
            }
        }

        impl $crate::Code<$id_type> for $type_name {}
    };
    ($type_name:ty, $id_field:ident, $id_type:ty) => {
        impl ::std::fmt::Display for $type_name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}", self.$id_field())
            }
        }

        impl ::std::convert::From<$type_name> for $id_type {
            fn from(v: $type_name) -> Self {
                v.$id_field()
            }
        }

        impl $crate::Code<$id_type> for $type_name {}
    };

    ($type_name:ty, $id_field:ident) => {
        code_impl!($type_name, $id_field, 'static str, String, to_string);
    };

    ($type_name:ty) => {
        code_impl!($type_name, code, 'static str, String, to_string);
    };
}

#[macro_export]
macro_rules! fixed_length_code {
    ($type_name:ty, $length:literal) => {
        impl $crate::FixedLengthCode for $type_name {
            fn fixed_length() -> usize {
                $length
            }
        }
    };
}

#[macro_export]
macro_rules! variable_length_code {
    ($type_name:ty, $min:literal, $max:literal) => {
        impl $crate::VariableLengthCode for $type_name {
            fn min_length() -> usize {
                $min
            }
            fn max_length() -> usize {
                $max
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod error;
pub use error::CodeParseError;

#[cfg(feature = "build")]
#[macro_use]
pub mod build;
