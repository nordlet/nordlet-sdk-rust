pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1ReferenceVatResolveResponseRatesItemCategory {
    Standard,
    Reduced,
    SuperReduced,
    Parking,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1ReferenceVatResolveResponseRatesItemCategory {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Standard => serializer.serialize_str("standard"),
            Self::Reduced => serializer.serialize_str("reduced"),
            Self::SuperReduced => serializer.serialize_str("super_reduced"),
            Self::Parking => serializer.serialize_str("parking"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1ReferenceVatResolveResponseRatesItemCategory {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "standard" => Ok(Self::Standard),
            "reduced" => Ok(Self::Reduced),
            "super_reduced" => Ok(Self::SuperReduced),
            "parking" => Ok(Self::Parking),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1ReferenceVatResolveResponseRatesItemCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Standard => write!(f, "standard"),
            Self::Reduced => write!(f, "reduced"),
            Self::SuperReduced => write!(f, "super_reduced"),
            Self::Parking => write!(f, "parking"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
