pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1LedgerPeriodsLockResponseStatus {
    Open,
    Locked,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1LedgerPeriodsLockResponseStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Open => serializer.serialize_str("open"),
            Self::Locked => serializer.serialize_str("locked"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1LedgerPeriodsLockResponseStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "open" => Ok(Self::Open),
            "locked" => Ok(Self::Locked),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1LedgerPeriodsLockResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Open => write!(f, "open"),
            Self::Locked => write!(f, "locked"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
