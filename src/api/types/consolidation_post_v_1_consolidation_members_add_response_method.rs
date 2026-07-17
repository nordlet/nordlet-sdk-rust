pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1ConsolidationMembersAddResponseMethod {
    Full,
    Proportional,
    Equity,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1ConsolidationMembersAddResponseMethod {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Full => serializer.serialize_str("full"),
            Self::Proportional => serializer.serialize_str("proportional"),
            Self::Equity => serializer.serialize_str("equity"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1ConsolidationMembersAddResponseMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "full" => Ok(Self::Full),
            "proportional" => Ok(Self::Proportional),
            "equity" => Ok(Self::Equity),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1ConsolidationMembersAddResponseMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Full => write!(f, "full"),
            Self::Proportional => write!(f, "proportional"),
            Self::Equity => write!(f, "equity"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
