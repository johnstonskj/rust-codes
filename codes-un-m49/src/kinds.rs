/*!
Provides a classifier for UN M49 regions.
*/

use codes_common::error::unknown_value;
use std::{fmt::Display, str::FromStr};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This enumeration is derived from the column headings in the M49 downloadable CSV file.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum RegionKind {
    Global,
    Region,
    SubRegion,
    IntermediateRegion,
    Country,
    Area,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for RegionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Global => "Global",
                Self::Region => "Region",
                Self::SubRegion => "SubRegion",
                Self::IntermediateRegion => "IntermediateRegion",
                Self::Country => "Country",
                Self::Area => "Area",
            }
        )
    }
}

impl FromStr for RegionKind {
    type Err = super::RegionClassificationCodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Global" => Ok(Self::Global),
            "Region" => Ok(Self::Region),
            "SubRegion" => Ok(Self::SubRegion),
            "IntermediateRegion" => Ok(Self::IntermediateRegion),
            "Country" => Ok(Self::Country),
            "Area" => Ok(Self::Area),
            _ => Err(unknown_value("RegionKind", s)),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
