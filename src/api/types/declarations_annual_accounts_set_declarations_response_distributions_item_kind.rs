pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AnnualAccountsSetDeclarationsResponseDistributionsItemKind {
    Dividend,
    InterimDividend,
    Other,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AnnualAccountsSetDeclarationsResponseDistributionsItemKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Dividend => serializer.serialize_str("dividend"),
            Self::InterimDividend => serializer.serialize_str("interim_dividend"),
            Self::Other => serializer.serialize_str("other"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AnnualAccountsSetDeclarationsResponseDistributionsItemKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "dividend" => Ok(Self::Dividend),
            "interim_dividend" => Ok(Self::InterimDividend),
            "other" => Ok(Self::Other),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AnnualAccountsSetDeclarationsResponseDistributionsItemKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Dividend => write!(f, "dividend"),
            Self::InterimDividend => write!(f, "interim_dividend"),
            Self::Other => write!(f, "other"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
