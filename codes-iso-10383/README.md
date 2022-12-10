# Package codes-iso-10383

This package contains an implementation of the [ISO
10383](https://www.iso.org/standard/61067.html) Securities and related
financial instruments — Codes for exchanges and market identification (MIC)
specification.

ISO 10383 specifies a universal method of identifying exchanges, trading
platforms, regulated or non-regulated markets and trade reporting facilities
as sources of prices and related information in order to facilitate automated
processing.

It is intended for use in any application and communication for identification of places

* where a financial instrument is listed (place of official listing),
* where a related trade is executed (place of trade), and
* where trade details are reported (trade reporting facility).

Note that field descriptions are taken from ISO 10383 Market Identifier Codes - [Release 2.0 Factsheet](https://www.iso20022.org/sites/default/files/2022-11/ISO10383_MIC_Release_2_0_Factsheet_v2.pdf).

For notes on the design of the API, see the repository 
[README](https://github.com/johnstonskj/rust-codes/blob/main/README.md).

## Example

```rust
use codes_iso_10383::{Category, MarketIdCode, Status};

let market = MarketIdCode::XNCM;
assert_eq!(market.operating_code(), Some(MarketIdCode::XNAS));
assert_eq!(market.is_segment(), true);
assert_eq!(market.market_name(), "NASDAQ CAPITAL MARKET");
assert_eq!(market.status(), Status::Active);
assert_eq!(market.market_category_code(), Some(Category::NSPD));
```

## Features

By default only the `serde` feature is enabled, the [MarketIdCode::code] and
[MarketIdCode::operating_code], and [MarketIdCode::is_segment] methods cannot be excluded.

* `serde` - Enables serialization of the [MarketIdCode] type.
* `market_name` - Adds the [MarketIdCode::market_name] method.
* `location` - Adds the [MarketIdCode::country_code] and [MarketIdCode::city] methods.
* `legal_entity` - Adds the [MarketIdCode::legal_entity_id] and [MarketIdCode::legal_entity_name] methods.
* `real_url - Uses the `Url` type from the `url` crate for the [MarketIdCode::website_url] method.
* `dates` - Adds the [MarketIdCode::creation_date], [MarketIdCode::last_update_date], [MarketIdCode::last_validation_date], and [MarketIdCode::expiration_date] methods.
* `real_dates` - Used the `DateTime<Utc>` types from the `chrono` crate for date functions **Work In Progress**
* `comments` - Adds the [MarketIdCode::comments] method.

## Changes

**Version 0.1.4**

* Added all missing methods.

**Version 0.1.3**

* Using new `codes-common` CSV handling framework.

**Version 0.1.2**

* Added `ALL_CODES` constant.

**Version 0.1.1**

* Clean-up implementation based on newer `codes-common`
* Using new `Code` trait and macro-created implementation.

**Version 0.1.0**

* Initial release

## TODO

TBD
