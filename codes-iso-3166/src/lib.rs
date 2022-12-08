/*!
This package contains an implementation of the [ISO 3166](https://www.iso.org/iso-3166-country-codes.html) part 1 standard.

The purpose of ISO 3166 is to define internationally recognized codes of
letters and/or numbers that we can use when we refer to countries and their
subdivisions. However, it does not define the names of countries – this
information comes from United Nations sources (Terminology Bulletin Country
Names and the Country and Region Codes for Statistical Use maintained by the
United Nations Statistics Divisions).

The country codes can be represented either as a two-letter code (alpha-2)
which is recommended as the general-purpose code, a three-letter code
(alpha-3) which is more closely related to the country name and a three-digit
numeric code (numeric-3) which can be useful if you need to avoid using Latin
script.

# Example

This example shows the use of the part-1 Country Code.

```rust
use codes_iso_3166::part_1::CountryCode;
use std::str::FromStr;

let country = CountryCode::from_str("AG").unwrap();

assert_eq!(country.alpha_2_code(), "AG");
assert_eq!(country.short_name(), "Antigua and Barbuda");
```

# Features

By default only the `serde` feature is enabled, and [part_1] two-letter
language codes.

* `serde` - Enables serialization of the different Language Code types.
* `alpha_3_code` - Adds the `CountryCode::alpha_3_code` method.
* `numeric_code` - Adds the `CountryCode::numeric_code` method.
* `independent` - Adds the `CountryCode::independent` method.
* `status` - Adds the `CountryCode::status` method.
* `full_name` - Adds the `CountryCode::full_name` method.
* `languages`  - Adds the `CountryCode::administrative_language` and `CountryCode::languages` methods (requires package `codes-iso-639`).
* `part_2` - Adds the corresponding module and `SubdivisionCode`.
  * `categories` - Adds the `SubdivisionCode::category_code` method and `SubdivisionCategoryCode` type.
  * `territories` - Adds the `TerritoryCode` type.
  * `languages` - Adds the `SubdivisionCode::name_language` method.

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

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub use codes_common::CodeParseError as CountryCodeError;

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod part_1;

#[cfg(feature = "part_2")]
pub mod part_2;
