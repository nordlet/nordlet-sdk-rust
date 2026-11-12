pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VatResolveReferenceRequestSide {
    Sale,
    Purchase,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for VatResolveReferenceRequestSide {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Sale => serializer.serialize_str("sale"),
            Self::Purchase => serializer.serialize_str("purchase"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for VatResolveReferenceRequestSide {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "sale" => Ok(Self::Sale),
            "purchase" => Ok(Self::Purchase),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for VatResolveReferenceRequestSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sale => write!(f, "sale"),
            Self::Purchase => write!(f, "purchase"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
