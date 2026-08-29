pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1InventoryLandedCostsListResponseRowsItemMethod {
    ByValue,
    ByQuantity,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1InventoryLandedCostsListResponseRowsItemMethod {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ByValue => serializer.serialize_str("by_value"),
            Self::ByQuantity => serializer.serialize_str("by_quantity"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1InventoryLandedCostsListResponseRowsItemMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "by_value" => Ok(Self::ByValue),
            "by_quantity" => Ok(Self::ByQuantity),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1InventoryLandedCostsListResponseRowsItemMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ByValue => write!(f, "by_value"),
            Self::ByQuantity => write!(f, "by_quantity"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
