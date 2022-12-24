use crate::CountryCodeError;
use codes_common::error::unknown_value;
use std::fmt::Display;
use std::str::FromStr;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Status assigned to each [super::CountryCode]. Note that those that are marked as
/// *formerly used* are not included directly.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Status {
    OfficiallyAssigned,
    ExceptionallyReserved,
    IndeterminatelyReserved,
    TransitionallyReserved,
    FormerlyUsed,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::OfficiallyAssigned => "OfficiallyAssigned",
                Self::ExceptionallyReserved => "ExceptionallyReserved",
                Self::IndeterminatelyReserved => "IndeterminatelyReserved",
                Self::TransitionallyReserved => "TransitionallyReserved",
                Self::FormerlyUsed => "FormerlyUsed",
            }
        )
    }
}

impl FromStr for Status {
    type Err = CountryCodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "OfficiallyAssigned" => Ok(Self::OfficiallyAssigned),
            "ExceptionallyReserved" => Ok(Self::ExceptionallyReserved),
            "IndeterminatelyReserved" => Ok(Self::IndeterminatelyReserved),
            "TransitionallyReserved" => Ok(Self::TransitionallyReserved),
            "FormerlyUsed" => Ok(Self::FormerlyUsed),
            _ => Err(unknown_value("Status", s)),
        }
    }
}
