# Project rust-codes

A family of packages to provide standard codes in an independent yet
structured manner

## Design Approach

**Tenets**

1. **Make them easy to understand**; wherever possible the structure of types
   and choice of names should be consistent across packages.
1. **Make them composable**; packages should only model a single standard and
   should reuse others.
1. **Keep them up-to-date**; 

## Changes

**2022-12-06**

* Released the following:
  * `codes-iso-639`; an implementation of the ISO-4217 Language Codes standard.
  * `codes-iso-10383`; an implementation of the ISO-10383 Market Identification (MIC) standard.
  * `codes-iso-17442`; an implementation of the ISO-17442 Legal Entity Identifier (LEI) standard.

**2022-11-30**

* Released the following:
  * `codes-agency`; codes that identify standards agencies.
  * `codes-iso-4217`; an implementation of the ISO-4217 currency codes standard.

## TODO

### Code Standards

* ISO 3166, Codes for the representation of names of countries and their subdivisions
* ISO 15924, Codes for the representation of names of scripts
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

### Data Types

* Vehicle identification number https://en.wikipedia.org/wiki/Vehicle_identification_number
* Serial shipping container code https://en.wikipedia.org/wiki/Serial_shipping_container_code
