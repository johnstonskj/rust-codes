# Project rust-codes

A family of packages to provide standard codes in an independent yet
structured manner

![MIT License](https://img.shields.io/badge/license-mit-118811.svg)
[![Rust](https://github.com/johnstonskj/rust-codes/actions/workflows/rust.yml/badge.svg)](https://github.com/johnstonskj/rust-codes/actions/workflows/rust.yml)
[![Audit](https://github.com/johnstonskj/rust-codes/actions/workflows/audit.yml/badge.svg)](https://github.com/johnstonskj/rust-codes/actions/workflows/audit.yml)
[![Codecov](https://codecov.io/gh/johnstonskj/rust-codes/branch/main/graph/badge.svg?token=1HGN6M4KIT)](https://codecov.io/gh/johnstonskj/rust-codes)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/rust-codes.svg)](https://github.com/johnstonskj/rust-codes/stargazers)

## Design Approach

**Tenets**

1. **Make them easy to understand**; wherever possible the structure of types
   and choice of names should be consistent across packages.
1. **Make them composable**; packages should only model a single standard and
   should reuse others.
1. **Keep them up-to-date**; 

## Changes

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

* IANA Character Sets
  https://www.iana.org/assignments/character-sets/character-sets.xhtml
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
