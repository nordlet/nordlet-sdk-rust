pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1FleetVehiclesCreateResponseStatus {
    Active,
    Sold,
    Scrapped,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1FleetVehiclesCreateResponseStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Active => serializer.serialize_str("active"),
            Self::Sold => serializer.serialize_str("sold"),
            Self::Scrapped => serializer.serialize_str("scrapped"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1FleetVehiclesCreateResponseStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "active" => Ok(Self::Active),
            "sold" => Ok(Self::Sold),
            "scrapped" => Ok(Self::Scrapped),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1FleetVehiclesCreateResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Sold => write!(f, "sold"),
            Self::Scrapped => write!(f, "scrapped"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
