pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CertificatesUploadDeclarationsResponseRowsItemHealth {
    Ok,
    Expiring,
    Expired,
    Unknown,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CertificatesUploadDeclarationsResponseRowsItemHealth {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Ok => serializer.serialize_str("ok"),
            Self::Expiring => serializer.serialize_str("expiring"),
            Self::Expired => serializer.serialize_str("expired"),
            Self::Unknown => serializer.serialize_str("unknown"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CertificatesUploadDeclarationsResponseRowsItemHealth {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ok" => Ok(Self::Ok),
            "expiring" => Ok(Self::Expiring),
            "expired" => Ok(Self::Expired),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CertificatesUploadDeclarationsResponseRowsItemHealth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ok => write!(f, "ok"),
            Self::Expiring => write!(f, "expiring"),
            Self::Expired => write!(f, "expired"),
            Self::Unknown => write!(f, "unknown"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
