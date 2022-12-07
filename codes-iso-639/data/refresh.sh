#!/usr/bin/env sh

curl -o iso-639-1.tsv "https://id.loc.gov/vocabulary/iso639-1.tsv"
curl -o iso-639-2.tsv "https://id.loc.gov/vocabulary/iso639-2.tsv"
curl -o iso-639-5.tsv "https://id.loc.gov/vocabulary/iso639-5.tsv"

curl -o iso-639-3.tsv "https://iso639-3.sil.org/sites/iso639-3/files/downloads/iso-639-3.tab"
curl -o iso-639-3-name-index.tsv "https://iso639-3.sil.org/sites/iso639-3/files/downloads/iso-639-3_Name_Index.tab"
curl -o iso-639-3-macro-languages.tsv "https://iso639-3.sil.org/sites/iso639-3/files/downloads/iso-639-3-macrolanguages.tab"
curl -o iso-639-3-retirements.tsv "https://iso639-3.sil.org/sites/iso639-3/files/downloads/iso-639-3_Retirements.tab"
