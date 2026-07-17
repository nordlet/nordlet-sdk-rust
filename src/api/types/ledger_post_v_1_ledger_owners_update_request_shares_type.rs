pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1LedgerOwnersUpdateRequestSharesType {
    V,
    Pr,
    Pp,
    Prv,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1LedgerOwnersUpdateRequestSharesType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::V => serializer.serialize_str("V"),
            Self::Pr => serializer.serialize_str("PR"),
            Self::Pp => serializer.serialize_str("PP"),
            Self::Prv => serializer.serialize_str("PRV"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1LedgerOwnersUpdateRequestSharesType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "V" => Ok(Self::V),
            "PR" => Ok(Self::Pr),
            "PP" => Ok(Self::Pp),
            "PRV" => Ok(Self::Prv),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1LedgerOwnersUpdateRequestSharesType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::V => write!(f, "V"),
            Self::Pr => write!(f, "PR"),
            Self::Pp => write!(f, "PP"),
            Self::Prv => write!(f, "PRV"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
