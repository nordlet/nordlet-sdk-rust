pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1DeclarationsLtSdGenerateRequestType {
    OneSd,
    TwoSd,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1DeclarationsLtSdGenerateRequestType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::OneSd => serializer.serialize_str("1-SD"),
            Self::TwoSd => serializer.serialize_str("2-SD"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1DeclarationsLtSdGenerateRequestType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "1-SD" => Ok(Self::OneSd),
            "2-SD" => Ok(Self::TwoSd),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1DeclarationsLtSdGenerateRequestType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OneSd => write!(f, "1-SD"),
            Self::TwoSd => write!(f, "2-SD"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
