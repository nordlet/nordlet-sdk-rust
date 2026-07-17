pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1AssetsAssetsGetResponseStatus {
    Active,
    FullyDepreciated,
    Disposed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1AssetsAssetsGetResponseStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Active => serializer.serialize_str("active"),
            Self::FullyDepreciated => serializer.serialize_str("fully_depreciated"),
            Self::Disposed => serializer.serialize_str("disposed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1AssetsAssetsGetResponseStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "active" => Ok(Self::Active),
            "fully_depreciated" => Ok(Self::FullyDepreciated),
            "disposed" => Ok(Self::Disposed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1AssetsAssetsGetResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::FullyDepreciated => write!(f, "fully_depreciated"),
            Self::Disposed => write!(f, "disposed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
