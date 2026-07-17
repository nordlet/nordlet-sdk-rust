pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1SalesActsListResponseRowsItemType {
    Goods,
    Services,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1SalesActsListResponseRowsItemType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Goods => serializer.serialize_str("goods"),
            Self::Services => serializer.serialize_str("services"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1SalesActsListResponseRowsItemType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "goods" => Ok(Self::Goods),
            "services" => Ok(Self::Services),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1SalesActsListResponseRowsItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Goods => write!(f, "goods"),
            Self::Services => write!(f, "services"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
