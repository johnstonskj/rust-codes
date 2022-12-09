# Package codes-iso-15924

This package contains an implementation of the
[ISO 15924](https://www.iso.org/standard/81905.html) Information and documentation — Codes for the representation of names of scripts specification.

ISO 15924, Codes for the representation of names of scripts, is an
international standard defining codes for writing systems or scripts (a "set
of graphic characters used for the written form of one or more languages").
Each script is given both a four-letter code and a numeric code.

Where possible the codes are derived from ISO 639-2, where the name of a
script and the name of a language using the script are identical (example:
Gujarātī ISO 639 guj, ISO 15924 Gujr). Preference is given to the 639-2
Bibliographical codes, which is different from the otherwise often preferred
use of the Terminological codes.

4-letter ISO 15924 codes are incorporated into the IANA Language Subtag
Registry for IETF language tags and so can be used in file formats that make
use of such language tags. For example, they can be used in HTML and XML to
help Web browsers determine which typeface to use for foreign text. This way
one could differentiate, for example, between Serbian written in the Cyrillic
(sr-Cyrl) or Latin (sr-Latn) script, or mark romanized or transliterated text
as such.

For notes on the design of the API, see the repository 
[README](https://github.com/johnstonskj/rust-codes/blob/main/README.md).

# Example

```rust
use codes_iso_15924::ScriptCode;

let code = ScriptCode::Rohg;

assert_eq!(code.code(), "Rohg");
assert_eq!(code.numeric_code(), 167);
assert_eq!(code.name(), "Hanifi Rohingya");
assert_eq!(code.property_value_alias(), Some("Hanifi_Rohingya"));
assert_eq!(code.unicode_version(), "11.0");
assert_eq!(code.date_string(),"2017-11-21");
```

# Features

By default only the `serde` feature is enabled.

* `serde` - Enables serialization of the [ScriptCode] type.

## Changes

**Version 0.1.1**

* Using new `codes-common` CSV handling framework.

**Version 0.1.0**

* Initial release

## TODO

TBD
