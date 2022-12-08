/*!
Codes for the representation of names of countries and their subdivisions –
Part 1: Country codes, defines codes for the names of countries, dependent
territories, and special areas of geographical interest.

The standard employs a code of letters and numbers to represent the name of a
given geographical area in order to save time and energy when describing the
area, as well as to reduce the risk of description errors. The official name
of the standard is Codes for the representation of names of countries and
their subdivisions.

# Alpha-2 Codes

SO 3166-1 alpha-2 codes are two-letter country codes defined in ISO 3166-1,
part of the ISO 3166 standard[1] published by the International Organization
for Standardization (ISO), to represent countries, dependent territories, and
special areas of geographical interest. They are the most widely used of the
country codes published by ISO (the others being alpha-3 and numeric), and are
used most prominently for the Internet's country code top-level domains (with
a few exceptions). They are also used as country identifiers extending the
postal code when appropriate within the international postal system for paper
mail, and have replaced the previous one consisting one-letter codes. They
were first included as part of the ISO 3166 standard in its first edition in
1974.

# Alpha-3 Codes

ISO 3166-1 alpha-3 codes are three-letter country codes defined in ISO 3166-1,
part of the ISO 3166 standard published by the International Organization for
Standardization (ISO), to represent countries, dependent territories, and
special areas of geographical interest. They allow a better visual association
between the codes and the country names than the two-letter alpha-2 codes (the
third set of codes is numeric and hence offers no visual association). They
were first included as part of the ISO 3166 standard in its first edition in
1974.

# Numeric Codes

ISO 3166-1 numeric (or numeric-3) codes are three-digit country codes defined
in ISO 3166-1, part of the ISO 3166 standard published by the International
Organization for Standardization (ISO), to represent countries, dependent
territories, and special areas of geographical interest. They are similar to
the three-digit country codes developed and maintained by the United Nations
Statistics Division, from which they originate in its UN M.49 standard. They
were first included as part of the ISO 3166 standard in its second edition in
1981, but they were released by the United Nations Statistics Division since
as early as 1970.

An advantage of numeric codes over alphabetic codes is script (writing system)
independence. The ISO 3166-1 alphabetic codes (alpha-2 and alpha-3) use
letters from the 26-letter English alphabet and are suitable for languages
based on the Latin alphabet. For people and systems using non-Latin scripts
(such as Arabic or Japanese), the English alphabet may be unavailable or
difficult to use, understand, or correctly interpret. While numeric codes
overcome the problems of script dependence, this independence comes at the
cost of loss of mnemonic convenience.

|                         | AG                  | ID                        |
| ----------------------- | ------------------- | ------------------------- |
| **alpha-2 code**        | AG                  | ID                        |
| **short name**          | Antigua and Barbuda | Indonesia                 |
| alpha-3 code            | ATG                 | IDN                       |
| numeric code            | 028                 | 360                       |
| independent             | true                | true                      |
| status                  | Assigned            | Assigned                  |
| full name               |                     | the Republic of Indonesia |
| languages               | Eng, Fra            | Eng, Fra, Ind             |
| administrative language | Eng                 | Ind                       |
*/

// ------------------------------------------------------------------------------------------------
//
// The rest of this file is generated by the package build script.
//
// ------------------------------------------------------------------------------------------------

include!(concat!(env!("OUT_DIR"), "/part_1.rs"));

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[cfg(feature = "status")]
#[doc(hidden)]
mod status;
#[cfg(feature = "status")]
pub use status::Status;
