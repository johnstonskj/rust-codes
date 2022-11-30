# Crate codes-agency

This package provides a common code representing standards agencies.

For notes on the design of the API, see the repository 
[README](https://github.com/johnstonskj/rust-codes/blob/main/README.md).

# Features

By default only the `serde` feature is enabled.

* `serde` - Enables serialization of the [CurrencyCode] type.
* `no-agency-url` - Removes the [Agency::url] method.
* `no-standard-url` - Removes the [Standard::url] methods.
  
## Changes

**Version 0.1.1**

* Initial documentation.

**Version 0.1.0**

* Initial place-holder release.

## TODO

1. Create a scheduled build that retrieves updated files from the ISO site.
