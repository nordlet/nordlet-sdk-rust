pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1BankFeedsConnectionsCompleteResponsePsuType {
    Business,
    Personal,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1BankFeedsConnectionsCompleteResponsePsuType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Business => serializer.serialize_str("business"),
            Self::Personal => serializer.serialize_str("personal"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1BankFeedsConnectionsCompleteResponsePsuType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "business" => Ok(Self::Business),
            "personal" => Ok(Self::Personal),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1BankFeedsConnectionsCompleteResponsePsuType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Business => write!(f, "business"),
            Self::Personal => write!(f, "personal"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
