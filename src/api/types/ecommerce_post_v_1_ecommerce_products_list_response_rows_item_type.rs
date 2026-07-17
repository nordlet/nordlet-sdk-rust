pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1EcommerceProductsListResponseRowsItemType {
    Product,
    Service,
    Set,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1EcommerceProductsListResponseRowsItemType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Product => serializer.serialize_str("product"),
            Self::Service => serializer.serialize_str("service"),
            Self::Set => serializer.serialize_str("set"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1EcommerceProductsListResponseRowsItemType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "product" => Ok(Self::Product),
            "service" => Ok(Self::Service),
            "set" => Ok(Self::Set),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1EcommerceProductsListResponseRowsItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Product => write!(f, "product"),
            Self::Service => write!(f, "service"),
            Self::Set => write!(f, "set"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
