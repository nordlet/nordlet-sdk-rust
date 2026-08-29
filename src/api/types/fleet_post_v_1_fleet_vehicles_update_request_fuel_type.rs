pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1FleetVehiclesUpdateRequestFuelType {
    Petrol,
    Diesel,
    Electric,
    Hybrid,
    Lpg,
    Other,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1FleetVehiclesUpdateRequestFuelType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Petrol => serializer.serialize_str("petrol"),
            Self::Diesel => serializer.serialize_str("diesel"),
            Self::Electric => serializer.serialize_str("electric"),
            Self::Hybrid => serializer.serialize_str("hybrid"),
            Self::Lpg => serializer.serialize_str("lpg"),
            Self::Other => serializer.serialize_str("other"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1FleetVehiclesUpdateRequestFuelType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "petrol" => Ok(Self::Petrol),
            "diesel" => Ok(Self::Diesel),
            "electric" => Ok(Self::Electric),
            "hybrid" => Ok(Self::Hybrid),
            "lpg" => Ok(Self::Lpg),
            "other" => Ok(Self::Other),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1FleetVehiclesUpdateRequestFuelType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Petrol => write!(f, "petrol"),
            Self::Diesel => write!(f, "diesel"),
            Self::Electric => write!(f, "electric"),
            Self::Hybrid => write!(f, "hybrid"),
            Self::Lpg => write!(f, "lpg"),
            Self::Other => write!(f, "other"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
