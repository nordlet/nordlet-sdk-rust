pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1SalesRecognitionModifyRequestApproach {
    Prospective,
    CumulativeCatchUp,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1SalesRecognitionModifyRequestApproach {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Prospective => serializer.serialize_str("prospective"),
            Self::CumulativeCatchUp => serializer.serialize_str("cumulative_catch_up"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1SalesRecognitionModifyRequestApproach {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "prospective" => Ok(Self::Prospective),
            "cumulative_catch_up" => Ok(Self::CumulativeCatchUp),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1SalesRecognitionModifyRequestApproach {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Prospective => write!(f, "prospective"),
            Self::CumulativeCatchUp => write!(f, "cumulative_catch_up"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
