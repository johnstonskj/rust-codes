use codes_common::{invalid_length, unknown_value};
use std::fmt;
use std::str::FromStr;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
///
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Status {
    /// Currently active
    Active,
    /// Expired, or deactivated
    Expired,
    /// Updated since last publication
    Updated,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl FromStr for Status {
    type Err = super::MarketIdCodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            Err(invalid_length("Status", s.len()))
        } else {
            match s {
                "ACTIVE" => Ok(Self::Active),
                "EXPIRED" => Ok(Self::Expired),
                "UPDATED" => Ok(Self::Updated),
                _ => Err(unknown_value("Status", s)),
            }
        }
    }
}

impl AsRef<str> for Status {
    fn as_ref(&self) -> &str {
        match self {
            Self::Active => "ACTIVE",
            Self::Expired => "EXPIRED",
            Self::Updated => "UPDATED",
        }
    }
}

impl From<Status> for String {
    fn from(v: Status) -> Self {
        v.as_ref().to_string()
    }
}
