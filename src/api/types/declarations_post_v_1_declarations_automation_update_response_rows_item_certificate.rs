pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1DeclarationsAutomationUpdateResponseRowsItemCertificate {
    Ok,
    Expiring,
    Expired,
    Unknown,
    Missing,
    NotNeeded,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1DeclarationsAutomationUpdateResponseRowsItemCertificate {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Ok => serializer.serialize_str("ok"),
            Self::Expiring => serializer.serialize_str("expiring"),
            Self::Expired => serializer.serialize_str("expired"),
            Self::Unknown => serializer.serialize_str("unknown"),
            Self::Missing => serializer.serialize_str("missing"),
            Self::NotNeeded => serializer.serialize_str("not-needed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1DeclarationsAutomationUpdateResponseRowsItemCertificate {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ok" => Ok(Self::Ok),
            "expiring" => Ok(Self::Expiring),
            "expired" => Ok(Self::Expired),
            "unknown" => Ok(Self::Unknown),
            "missing" => Ok(Self::Missing),
            "not-needed" => Ok(Self::NotNeeded),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1DeclarationsAutomationUpdateResponseRowsItemCertificate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ok => write!(f, "ok"),
            Self::Expiring => write!(f, "expiring"),
            Self::Expired => write!(f, "expired"),
            Self::Unknown => write!(f, "unknown"),
            Self::Missing => write!(f, "missing"),
            Self::NotNeeded => write!(f, "not-needed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
