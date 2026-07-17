pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1DeclarationsLtGpm313ComputeRequestPayoutTiming {
    SameMonth,
    NextMonth,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1DeclarationsLtGpm313ComputeRequestPayoutTiming {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::SameMonth => serializer.serialize_str("same-month"),
            Self::NextMonth => serializer.serialize_str("next-month"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1DeclarationsLtGpm313ComputeRequestPayoutTiming {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "same-month" => Ok(Self::SameMonth),
            "next-month" => Ok(Self::NextMonth),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1DeclarationsLtGpm313ComputeRequestPayoutTiming {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SameMonth => write!(f, "same-month"),
            Self::NextMonth => write!(f, "next-month"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
