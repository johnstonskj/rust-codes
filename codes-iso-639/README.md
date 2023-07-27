# Package codes-iso-639

This package contains an implementation of the ISO 639
[part 1](https://www.iso.org/standard/22109.html),
[part 2](https://www.iso.org/standard/4767.html),
[part 3](https://www.iso.org/standard/39534.html), and
[part 5](https://www.iso.org/standard/39536.html)
Language Code specifications.

[![crates.io](https://img.shields.io/crates/v/codes-iso-639.svg)](https://crates.io/crates/codes-iso-639)
[![docs.rs](https://docs.rs/codes-iso-639/badge.svg)](https://docs.rs/codes-iso-639)

ISO 639-1, the two-character code, was devised primarily for use in
terminology and includes identifiers for most of the major languages of the
world that are not only most frequently represented in the total body of the
world's literature, but that are also among the most developed languages of
the world, having specialized vocabulary and terminology. ISO 639-1 includes
identifiers for a subset of the languages covered by ISO 639-2.

ISO 639-2, the three-character code, was devised primarily for use in
bibliography, as well as in terminology. It has a less restrictive scope than
ISO 639-1, being devised to include identifiers for languages that are most
frequently represented in the total body of the world's literature, regardless
of whether specialized terminologies exist in those languages or not. Because
three characters allow for a much larger set of distinct identifiers, an
alpha-3 code can accommodate a much larger set of languages. Indeed, ISO 639-2
does include significantly more entries than ISO 639-1, yet the scope is not
so broad as to result in a separate identifier for every individual language
that has been documented. ISO 639-2 limits coverage of individual languages to
those for which at least modest bodies of literature have been developed.
Other languages are still accommodated, however, by means of identifiers for
collections of languages, such as language families.

In summary, the basic difference between ISO 639-1 and ISO 639-2 has to do
with scope: the scope of ISO 639-1 is more restrictive, focusing on languages
for which specialized terminologies have been developed. In practical terms,
ISO 639-2 covers a larger number of individual languages (due to its
less-restrictive scope). It also includes identifiers for collections of
languages.

For notes on the design of the API, see the repository 
[README](https://github.com/johnstonskj/rust-codes/blob/main/README.md).

## Example

```rust
use codes_iso_639::part_1::LanguageCode;

let code = LanguageCode::Fr;

assert_eq!(code.as_ref(), "fr");
assert_eq!(code.language_name(), "French");
```

## Features

By default only the `serde` feature is enabled, and part-1 two-letter
language codes.

* `serde` - Enables serialization of the different Language Code types.
* `part_3` - Adds the ISO 639-3 three-letter language codes.
  * `comment` - Adds the `LanguageCode::comment` method.
  * `language_type` - Adds the `LanguageCode::language_type` method.
  * `macro_individuals` - Adds the `LanguageCode::macro_individuals` method.
  * `scope` - Adds the `LanguageCode::scope` method.
* `part_5` - Adds the ISO 639-5 three-letter language family or group codes.

## Changes

**Version 0.1.5**

* Catering for new `build` module in codes-common

**Version 0.1.4**

* Implemented `Standardized` and `FixedLengthCode` traits.

**Version 0.1.3**

* Using new `codes-common` CSV handling framework.

**Version 0.1.2**

* Added `ALL_CODES` constant.

**Version 0.1.1**

* Clean-up implementation based on newer `codes-common`
* Using new `Code` trait and macro-created implementation.

**Version 0.1.0**

* Initial release, includes parts 1, 2, and 3.

## TODO

TBD
