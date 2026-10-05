pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LtPln204ComputeDeclarationsResponseVariant {
    Pln204,
    Pln204A,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for LtPln204ComputeDeclarationsResponseVariant {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Pln204 => serializer.serialize_str("PLN204"),
            Self::Pln204A => serializer.serialize_str("PLN204A"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for LtPln204ComputeDeclarationsResponseVariant {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "PLN204" => Ok(Self::Pln204),
            "PLN204A" => Ok(Self::Pln204A),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for LtPln204ComputeDeclarationsResponseVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pln204 => write!(f, "PLN204"),
            Self::Pln204A => write!(f, "PLN204A"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
