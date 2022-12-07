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

* Standards:
  * ISO 3166, Codes for the representation of names of countries and their subdivisions
  * ISO 15924, Codes for the representation of names of scripts
  * IANA Character Sets https://www.iana.org/assignments/character-sets/character-sets.xhtml
  * ISO ICS
    https://www.iso.org/files/live/sites/isoorg/files/archive/pdf/en/international_classification_for_standards.pdf
  * ISO/IEC 81346-2
  * SIC https://en.wikipedia.org/wiki/Standard_Industrial_Classification
  * NAICS https://www.census.gov/naics/
  * ITU-T standards E.123 and E.164. 
  * Currency symbols? https://en.wikipedia.org/wiki/Currency_symbol
  * ISO (SWIFT) BIC https://en.wikipedia.org/wiki/ISO_9362
  * ISO 13616 IBAN
