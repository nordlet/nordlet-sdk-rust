pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AssetsUpdateAssetsResponseDisposalReason {
    Sold,
    Scrapped,
    WrittenOff,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AssetsUpdateAssetsResponseDisposalReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Sold => serializer.serialize_str("sold"),
            Self::Scrapped => serializer.serialize_str("scrapped"),
            Self::WrittenOff => serializer.serialize_str("written_off"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AssetsUpdateAssetsResponseDisposalReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "sold" => Ok(Self::Sold),
            "scrapped" => Ok(Self::Scrapped),
            "written_off" => Ok(Self::WrittenOff),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AssetsUpdateAssetsResponseDisposalReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sold => write!(f, "sold"),
            Self::Scrapped => write!(f, "scrapped"),
            Self::WrittenOff => write!(f, "written_off"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
