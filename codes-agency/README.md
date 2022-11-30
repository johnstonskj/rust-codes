# Crate codes-agency

This package provides a common code representing standards agencies.

The two core types, [Agency] and [Standard] work together to provide reporting
capabilities to other *codes* project packages. Specifically a package that
provides types corresponding to a standard definition can have an instance of
the [Standard] struct that describes the standard. This in turn references the
[Agency] that controls the standard.

Consider the following example from the ISO 4217 package.

```rust
use codes_iso_4217::ISO_4217;

assert_eq!(ISO_4217.short_ref(), "4217");
assert_eq!(ISO_4217.title(), "Currency codes");
assert_eq!(ISO_4217.agency().to_string(), String::from("ISO"));
```

For notes on the design of the API, see the repository 
[README](https://github.com/johnstonskj/rust-codes/blob/main/README.md).

# Features

By default only the `serde` feature is enabled.

* `serde` - Enables serialization of the [CurrencyCode] type.
* `no-agency-url` - Removes the [Agency::url] method.
* `no-standard-url` - Removes the [Standard::url] methods.
  
## Changes

**Version 0.1.2**

* Remove the trait `Code`.

**Version 0.1.1**

* Initial documentation.

**Version 0.1.0**

* Initial place-holder release.

## TODO

1. Create a scheduled build that retrieves updated files from the ISO site.
