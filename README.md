# Project rust-codes

A family of packages to provide standard codes in an independent yet
structured manner

![MIT License](https://img.shields.io/badge/license-mit-118811.svg)
[![Rust](https://github.com/johnstonskj/rust-codes/actions/workflows/rust.yml/badge.svg)](https://github.com/johnstonskj/rust-codes/actions/workflows/rust.yml)
[![Security audit](https://github.com/johnstonskj/rust-codes/actions/workflows/security-audit.yml/badge.svg)](https://github.com/johnstonskj/rust-codes/actions/workflows/security-audit.yml)
[![Codecov](https://codecov.io/gh/johnstonskj/rust-codes/branch/main/graph/badge.svg?token=1HGN6M4KIT)](https://codecov.io/gh/johnstonskj/rust-codes)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/rust-codes.svg)](https://github.com/johnstonskj/rust-codes/stargazers)

## Supported Standards

| Agency | Standard | Description                                                                                    |
|--------|----------|------------------------------------------------------------------------------------------------|
| GS1    | GLN      | Global Location Number (GLN)                                                                   |
| IANA   | Charset  | IANA Character Sets                                                                            |
| ISO    | 639      | Language Codes                                                                                 |
| ISO    | 3166     | Country and Subdivision Codes                                                                  |
| ISO    | 4217     | Currency codes                                                                                 |
| ISO    | 6166     | International securities identification number (ISIN)                                          |
| ISO    | 10383    | Market Identification (MIC)                                                                    |
| ISO    | 15924    | Information and documentation — Codes for the representation of names of scripts specification |
| ISO    | 17442    | Legal Entity Identifier (LEI)                                                                  |
| UN     | M49      | Region Codes                                                                                   |

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
  or name as a variant. The [codes-iso-4217](https://docs.rs/codes-iso-4217)
  package is an example of this pattern.
* **Constant Numeric Type**; these are cases where the standard defines a
  clearly enumerated set of numeric values that identify specific codes. In
  this case we use a
  [newtype](https://doc.rust-lang.org/rust-by-example/generics/new_types.html)
  to capture the specific numeric identifiers and a set of defined constant
  instances. The [codes-iana-charset](https://docs.rs/codes-iana-charset)
  package is an example of this pattern.
* **Non-Enumerated Type**; these are cases there the standard does not define
  values, but addresses the format, or structure, of a code or identifier. For
  example the Legal Entity ID specification describes the format of an
  identifier but cannot enumerate all possible values. The
  [codes-iso-17442](https://docs.rs/codes-iso-17442) package is an example of
  this pattern.

For information on contributing to this project, see the following.

1. Project [Code-of-Conduct](CODE_OF_CONDUCT.md).
1. Project [Contribution Guidelines](CONTRIBUTING.md).

### Common Features

All codes, regardless of pattern implement the `codes-common::Code` trait
which simply acts as a set of required minimum capabilities.

``` rust
pub trait Code<T>: 
    Clone + Debug + Display + FromStr + Into<T> + PartialEq + Eq + Hash {}
```

Most types so far therefore have a definition including the following derived
implementations.

``` rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
/* ExampleCode */
```

Additionally, for non-copy types, it is common to implement `AsRef`.

Property, and therefore method, values should be as simple as possible and
given that most values are constructed at build time it is possible to use
static values. Where possible strings should be returned as `&'static str` and
numeric values as appropriate. 

In some cases it would be valuable to return a parsed, or extended, value type
instead of a simple string. For example the method `website_url` could return
the `url::Url` type, or `creation_date` could return `chrono::DateTime`.
However, these should not necessarily be required and so gating the extended
type version with a feature is a recommended practice.

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

Accessor methods should also be marked **`const`** as much as possible.

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

Finally, the `codes-common` package defines an error that has specific
variants useful for `FromStr` implementations and may be re-exported by
specific standard packages.

``` rust
pub use codes_common::CodeParseError as ExampleCodeError;
```

### Named Enumeration Pattern

The data type for this pattern looks like any standard Rust enum, and should
provide some meaningful doc comment. In many cases the common names for
variants do not conform to Rust naming conventions such as the two-letter
country code "US" or the three-letter currency code "USD", this is preferred
over forcing commonly used uppercase identifiers into Camel case.

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
```

The implementation of `FromStr` will use the values `ExampleOne`, `ExampleTwo`, etc.

An array is provided for each code type that contains all values, this is
particularly useful for creation of indexes and filter functions by iterating
over this value.

``` rust
pub const ALL_CODES: [ExampleCode;4] = [
    ExampleCode::ExampleOne,
    ExampleCode::ExampleTwo[,
    ExampleCode::ExampleThree,
    ExampleCode::ExampleFour,
];
```

### Constant Numeric Pattern

The data type for this pattern is a simple newtype struct wrapping an
appropriate numerical type. The default type for integer values is `u16`.

``` rust
pub struct ExampleCode(u16);
```

For numeric code types there should also be an implementation of `TryFrom` to
convert from numeric values into the code type.

``` rust
impl TryFrom<u16> for ExampleCode {
    // ...
```

For ease of use, each defined value is represented as a constant value. The
naming of these constants should have a meaningful prefix which may be the
standard name or some derived form.

``` rust
pub const EXAMPLE_1: ExampleCode = ExampleCode(1);
```

Finally, an equivalent `ALL_CODES` array is provided.

``` rust
pub const ALL_CODES: [ExampleCode;4] = [
    EXAMPLE_1,
    EXAMPLE_2,
    EXAMPLE_3,
    EXAMPLE_4,
];
```

### Non-Enumerated Type Pattern

Whenever possible the use of `FromStr` or `TryFrom` should be used over
explicit constructor functions. If an explicit function is required it should
be `new`, `new_with...`, or `new_from`.

``` rust
impl ExampleCode {
    pub const fn is_valid(s: &'static str) -> bool {
        todo!()
    }
}
```

At times it is useful to have a constructor that *does not* perform validity
checks. This is particularly useful within a package for constructing constant
values and so forth. In this case the name `new_unchecked` is used to allow
for the function to stand out when used.

``` rust
impl ExampleCode {
    #[doc(hidden)]
    pub(crate) const fn new_unchecked(s: &'static str) -> Self {
        todo!()
    }
}
```

It is not required that this function is private, but care should be taken in
making *unchecked* operations public.

## Changes

**2022-12-23**

* Moved all check digit calculations to a new package, imaginatively
  named `codes-check-digits`.
* Added new trait `Standardized` to the `codes-agency` package to allow
  standard types to return an instance of the `Standard` struct.
* Added new traits `FixedLengthCode` and `VariableLengthCode` to the 
  `codes-check-digits` package to tag types that implement `Code`.

**2022-12-19**

* Released the following:
  * [`codes-iso-6166`](codes-iso-6166); an implementation of the [ISO 6166](https://www.iso.org/standard/78502.html)
  International securities identification number (ISIN) standard.

**2022-12-14**

* Released the following:
  * [`codes-gs1-gln`](codes-gs1-gln); an implementation of the [GS1 GLN](https://www.gs1.org/standards/id-keys/gln) 
  Global Location Number (GLN) specification.
 
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
* UN/LOCODE	United Nations Code for Trade and Transport Locations
* ISO/IEC 7812 Identification cards – Identification of issuers
  https://en.wikipedia.org/wiki/ISO/IEC_7812

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
* ISO 10962, generally known as CFI (Classification of Financial Instruments)
  https://en.wikipedia.org/wiki/ISO_10962


https://unstats.un.org/unsd/classifications/unsdclassifications/

### Data Types

* Vehicle identification number https://en.wikipedia.org/wiki/Vehicle_identification_number
* Serial shipping container code https://en.wikipedia.org/wiki/Serial_shipping_container_code

### Other

* ISO/IEC 7064:2003 (https://www.iso.org/standard/31531.html)
  Information technology — Security techniques — Check character systems
