pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VatResolveReferenceRequestServiceKind {
    ShortTermAccommodation,
    PassengerRoadTransport,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for VatResolveReferenceRequestServiceKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ShortTermAccommodation => serializer.serialize_str("short_term_accommodation"),
            Self::PassengerRoadTransport => serializer.serialize_str("passenger_road_transport"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for VatResolveReferenceRequestServiceKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "short_term_accommodation" => Ok(Self::ShortTermAccommodation),
            "passenger_road_transport" => Ok(Self::PassengerRoadTransport),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for VatResolveReferenceRequestServiceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ShortTermAccommodation => write!(f, "short_term_accommodation"),
            Self::PassengerRoadTransport => write!(f, "passenger_road_transport"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
