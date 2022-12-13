# Project rust-codes

A family of packages to provide standard codes in an independent yet
structured manner

![MIT License](https://img.shields.io/badge/license-mit-118811.svg)
[![Rust](https://github.com/johnstonskj/rust-codes/actions/workflows/rust.yml/badge.svg)](https://github.com/johnstonskj/rust-codes/actions/workflows/rust.yml)
[![Security audit](https://github.com/johnstonskj/rust-codes/actions/workflows/security-audit.yml/badge.svg)](https://github.com/johnstonskj/rust-codes/actions/workflows/security-audit.yml)
[![Codecov](https://codecov.io/gh/johnstonskj/rust-codes/branch/main/graph/badge.svg?token=1HGN6M4KIT)](https://codecov.io/gh/johnstonskj/rust-codes)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/rust-codes.svg)](https://github.com/johnstonskj/rust-codes/stargazers)

## Design Approach

**Tenets**

1. **Make them easy to understand**; wherever possible the structure of types
   and choice of names should be consistent across packages.
1. **Make them composable**; packages should only model a single standard and
   should reuse others.
1. **Keep them up-to-date**; find ways to automate updates from source material.

So far there are three distinct patterns used when implementing codes, namely:

* **Named Enumeration Type**; these are cases where the standard defines a
  clearly enumerated set of values commonly referred to by some non-numeric
  identifier or name. The code type is a Rust **`enum`** with each identifier
  or name as a variant.
* **Constant Numeric Type**; these are cases where the standard defines a
  clearly enumerated set of numeric values that identify specific codes. In
  this case we use a
  [newtype](https://doc.rust-lang.org/rust-by-example/generics/new_types.html)
  to capture the specific numeric identifiers and a set of defined constant
  instances.
* **Non-Enumerated Type**; these are cases there the standard does not define
  values, but addresses the format, or structure, of a code or identifier. For
  example the Legal Entity ID specification describes the format of an
  identifier but cannot enumerate all possible values.


For information on contributing to this project, see the following.

1. Project [Code-of-Conduct](CODE_OF_CONDUCT.md).
1. Project [Contribution Guidelines](CONTRIBUTING.md).

### Common Features

Implemented traits `codes-common::Code`

``` rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
/* ExampleCode */
```

* `Display`
* `FromStr`
* `From`

* `AsRef`

Value Types

``` rust

```

Extended Value Types

* `Url`
* `DateTime`


```rust
impl ExampleCode {
    #[cfg(feature = "real_url")]
    pub fn website_url(&self) -> Option<url::Url> {
        todo!()
    }
    #[cfg(not(feature = "real_url"))]
    pub const fn website_url(&self) -> Option<&'static str> {
        todo!()
    }
}
```

Methods

Accessor methods should be marked **`const`** as much as possible.

``` rust
impl ExampleCode {
     pub const fn code(&self) -> &'static str {
         match self {
             Self::ExampleOne => "001",
             Self::ExampleTwo => "002",
             // ...
         }
     }
}
```

Errors

``` rust
pub use codes_common::CodeParseError as ExampleCodeError;
```

### Named Enumeration Pattern

See [codes-iso-4217](https://docs.rs/codes-iso-4217)

Data Type

``` rust
pub enum ExampleCode {
    /// Example number one
    ExampleOne,
    /// Example number two
    ExampleTwo,
    /// Example number three
    ExampleThree,
    /// Example number four
    ExampleFour,
}

`FromStr` will use the values `ExampleOne`, `ExampleTwo`, etc.

```

All Codes array

``` rust
pub const ALL_CODES: [ExampleCode;4] = [
    ExampleCode::ExampleOne,
    ExampleCode::ExampleTwo[,
    ExampleCode::ExampleThree,
    ExampleCode::ExampleFour,
];
```

### Constant Numeric Pattern

See [codes-iana-charset](https://docs.rs/codes-iana-charset)

Data Type

``` rust
pub struct ExampleCode(u16);

```

* `TryFrom`

Constant Values

``` rust
pub const EXAMPLE_1: ExampleCode = ExampleCode(1);
```

All Codes array

``` rust
pub const ALL_CODES: [ExampleCode;4] = [
    EXAMPLE_1,
    EXAMPLE_2,
    EXAMPLE_3,
    EXAMPLE_4,
];
```

### Non-Enumerated Type Pattern

See [codes-iso-17442](https://docs.rs/codes-iso-17442)

Constructors

where possible use FromStr or tryFrom. Else use `new`

``` rust
impl ExampleCode {
    #[doc(hidden)]
    pub(crate) const fn new_unchecked(s: &'static str) -> Self {
        todo!()
    }
}
```


## Changes

**2022-12-12**

* Released the following:
  * [`codes-iana-charset`](codes-iana-charset); an implementation of the [IANA Character
  Sets](https://www.iana.org/assignments/character-sets/character-sets.xhtml)
  standard.
  
**2022-12-09**

* Released the following:
  * [`codes-iso-15924`](codes-iso-15924); an implementation of the [ISO 15924](https://www.iso.org/standard/81905.html) Information and documentation — Codes for the representation of names of scripts specification.
  * [`codes-un-m49`](codes-un-m49); an implementation of the [UN M49](https://unstats.un.org/unsd/methodology/m49/overview) Region Codes standard.
  * Updated [`codes-iso-10383`](codes-iso-10383) with all methods.
  
**2022-12-09**

* Released the following:
  * [`codes-iso-3166`](codes-iso-3166); an implementation of the [ISO 3166](https://www.iso.org/iso-3166-country-codes.html) Country and
    Subdivision Codes standard.

**2022-12-06**

* Released the following:
  * [`codes-iso-639`](codes-iso-639); an implementation of the [ISO 4217](https://www.iso.org/iso-639-language-codes.html) Language Codes standard.
  * [`codes-iso-10383`](codes-iso-10383); a partial implementation of the [ISO 10383](https://www.iso.org/standard/61067.html) Market Identification (MIC) standard.
  * [`codes-iso-17442`](https://www.iso.org/standard/75998.html); an implementation of the [ISO 17442](https://www.iso.org/standard/78829.html) Legal Entity Identifier (LEI) standard.

**2022-11-30**

* Released the following:
  * [`codes-agency`](codes-agency); codes that identify standards agencies.
  * [`codes-iso-4217`](codes-iso-4217); an implementation of the [ISO 4217](https://www.iso.org/iso-4217-currency-codes.html) Currency codes standard.

## TODO

### Code Standards

* ISO (SWIFT) BIC https://en.wikipedia.org/wiki/ISO_9362
* ISO 13616 IBAN
* Currency symbols? https://en.wikipedia.org/wiki/Currency_symbol
* ISO 3901	International Standard Recording Code (ISRC)
* ISO 6166 International Securities Identifying Number (ISIN)
* UN/LOCODE	United Nations Code for Trade and Transport Locations

### Classification Standards

* ISO ICS (https://github.com/metanorma/iso-ics-codes) and
  https://www.iso.org/files/live/sites/isoorg/files/archive/pdf/en/international_classification_for_standards.pdf
* IEC 81346-1:2022 Industrial systems, installations and equipment and 
  industrial products — Structuring principles and reference 
  designations — Part 1: Basic rules
* ISO/IEC 81346-2 - Industrial systems, installations and equipment and
  industrial products — Structuring principles and reference 
  designations — Part 2: Classification of objects and codes for classes
* SIC https://en.wikipedia.org/wiki/Standard_Industrial_Classification
* NAICS https://www.census.gov/naics/ and https://en.wikipedia.org/wiki/North_American_Industry_Classification_System
* ITU-T E.123 Notation for national and international telephone
  numbers, e-mail addresses and web addresses
* ITU-T E.164 The international public telecommunication numbering plan
* UN number - A UN number (United Nations number) is a four-digit number 
  that identifies hazardous materials, and articles (such as explosives, 
  flammable liquids, oxidizers, toxic liquids, etc.) in the framework 
  of international trade and transport.
* United Nations Standard Products and Services Code (UNSPSC) https://en.wikipedia.org/wiki/UNSPSC

https://unstats.un.org/unsd/classifications/unsdclassifications/

### Data Types

* Vehicle identification number https://en.wikipedia.org/wiki/Vehicle_identification_number
* Serial shipping container code https://en.wikipedia.org/wiki/Serial_shipping_container_code
