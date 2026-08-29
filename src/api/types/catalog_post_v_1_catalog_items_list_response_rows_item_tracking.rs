pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1CatalogItemsListResponseRowsItemTracking {
    None,
    Lot,
    Serial,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1CatalogItemsListResponseRowsItemTracking {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::None => serializer.serialize_str("none"),
            Self::Lot => serializer.serialize_str("lot"),
            Self::Serial => serializer.serialize_str("serial"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1CatalogItemsListResponseRowsItemTracking {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "none" => Ok(Self::None),
            "lot" => Ok(Self::Lot),
            "serial" => Ok(Self::Serial),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1CatalogItemsListResponseRowsItemTracking {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::Lot => write!(f, "lot"),
            Self::Serial => write!(f, "serial"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
