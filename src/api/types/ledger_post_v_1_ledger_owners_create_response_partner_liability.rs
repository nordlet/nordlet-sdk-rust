pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1LedgerOwnersCreateResponsePartnerLiability {
    General,
    Limited,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1LedgerOwnersCreateResponsePartnerLiability {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::General => serializer.serialize_str("general"),
            Self::Limited => serializer.serialize_str("limited"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1LedgerOwnersCreateResponsePartnerLiability {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "general" => Ok(Self::General),
            "limited" => Ok(Self::Limited),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1LedgerOwnersCreateResponsePartnerLiability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::General => write!(f, "general"),
            Self::Limited => write!(f, "limited"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
