pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1DeclarationsEuSmeThresholdGetResponseStatus {
    NotApplicable,
    Below,
    Approaching,
    Exceeded,
    Unknown,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1DeclarationsEuSmeThresholdGetResponseStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::NotApplicable => serializer.serialize_str("not_applicable"),
            Self::Below => serializer.serialize_str("below"),
            Self::Approaching => serializer.serialize_str("approaching"),
            Self::Exceeded => serializer.serialize_str("exceeded"),
            Self::Unknown => serializer.serialize_str("unknown"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1DeclarationsEuSmeThresholdGetResponseStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "not_applicable" => Ok(Self::NotApplicable),
            "below" => Ok(Self::Below),
            "approaching" => Ok(Self::Approaching),
            "exceeded" => Ok(Self::Exceeded),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1DeclarationsEuSmeThresholdGetResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotApplicable => write!(f, "not_applicable"),
            Self::Below => write!(f, "below"),
            Self::Approaching => write!(f, "approaching"),
            Self::Exceeded => write!(f, "exceeded"),
            Self::Unknown => write!(f, "unknown"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
