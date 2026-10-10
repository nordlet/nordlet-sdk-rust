pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreatePlatformSellersRequestKind {
    Individual,
    Entity,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreatePlatformSellersRequestKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Individual => serializer.serialize_str("individual"),
            Self::Entity => serializer.serialize_str("entity"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreatePlatformSellersRequestKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "individual" => Ok(Self::Individual),
            "entity" => Ok(Self::Entity),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreatePlatformSellersRequestKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Individual => write!(f, "individual"),
            Self::Entity => write!(f, "entity"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
