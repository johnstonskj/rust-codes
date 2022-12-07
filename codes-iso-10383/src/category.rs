use codes_common::{invalid_length, unknown_value};
use std::fmt;
use std::str::FromStr;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The Market Category specifies the type of market. The list of market types
/// is predefined.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Category {
    /// Alternative Trading System
    ATSS,
    /// Approved Publication Arrangement
    APPA,
    /// Approved Reporting Mechanism
    ARMS,
    /// Consolidated Tape Provider
    CTPS,
    /// Crypto Asset Services Provider
    CASP,
    /// Designated Contract Market
    DCMS,
    /// Inter-Dealer Quotation System
    IDQS,
    /// Multilateral Trading Facility
    MLTF,
    /// Not Specified
    NSPD,
    /// Organised Trading Facility
    OTFS,
    /// Other
    OTHR,
    /// Recognised Market Operator
    RMOS,
    /// Regulated Market
    RMKT,
    /// Swap Execution Facility
    SEFS,
    /// Systematic Internaliser
    SINT,
    /// Trade Reporting Facility
    TRFS,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl FromStr for Category {
    type Err = super::MarketIdCodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            Err(invalid_length("Category", s.len()))
        } else {
            match s {
                "ATSS" => Ok(Self::ATSS),
                "APPA" => Ok(Self::APPA),
                "ARMS" => Ok(Self::ARMS),
                "CTPS" => Ok(Self::CTPS),
                "CASP" => Ok(Self::CASP),
                "DCMS" => Ok(Self::DCMS),
                "IDQS" => Ok(Self::IDQS),
                "MLTF" => Ok(Self::MLTF),
                "NSPD" => Ok(Self::NSPD),
                "OTFS" => Ok(Self::OTFS),
                "OTHR" => Ok(Self::OTHR),
                "RMOS" => Ok(Self::RMOS),
                "RMKT" => Ok(Self::RMKT),
                "SEFS" => Ok(Self::SEFS),
                "SINT" => Ok(Self::SINT),
                "TRFS" => Ok(Self::TRFS),
                _ => Err(unknown_value("Category", s)),
            }
        }
    }
}

impl AsRef<str> for Category {
    fn as_ref(&self) -> &str {
        match self {
            Self::ATSS => "ATSS",
            Self::APPA => "APPA",
            Self::ARMS => "ARMS",
            Self::CTPS => "CTPS",
            Self::CASP => "CASP",
            Self::DCMS => "DCMS",
            Self::IDQS => "IDQS",
            Self::MLTF => "MLTF",
            Self::NSPD => "NSPD",
            Self::OTFS => "OTFS",
            Self::OTHR => "OTHR",
            Self::RMOS => "RMOS",
            Self::RMKT => "RMKT",
            Self::SEFS => "SEFS",
            Self::SINT => "SINT",
            Self::TRFS => "TRFS",
        }
    }
}

impl From<Category> for String {
    fn from(v: Category) -> Self {
        v.as_ref().to_string()
    }
}

impl Category {
    #[cfg(feature = "category_description")]
    pub fn description(&self) -> &'static str {
        match self {
            Self::ATSS => "Alternative Trading System",
            Self::APPA => "Approved Publication Arrangement",
            Self::ARMS => "Approved Reporting Mechanism",
            Self::CTPS => "Consolidated Tape Provider",
            Self::CASP => "Crypto Asset Services Provider",
            Self::DCMS => "Designated Contract Market",
            Self::IDQS => "Inter-Dealer Quotation System",
            Self::MLTF => "Multilateral Trading Facility",
            Self::NSPD => "Not Specified",
            Self::OTFS => "Organised Trading Facility",
            Self::OTHR => "Other",
            Self::RMOS => "Recognised Market Operator",
            Self::RMKT => "Regulated Market",
            Self::SEFS => "Swap Execution Facility",
            Self::SINT => "Systematic Internaliser",
            Self::TRFS => "Trade Reporting Facility",
        }
    }
}
