pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1ConsolidationReportRequestCategory {
    Micro,
    Small,
    Medium,
    Large,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1ConsolidationReportRequestCategory {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Micro => serializer.serialize_str("micro"),
            Self::Small => serializer.serialize_str("small"),
            Self::Medium => serializer.serialize_str("medium"),
            Self::Large => serializer.serialize_str("large"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1ConsolidationReportRequestCategory {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "micro" => Ok(Self::Micro),
            "small" => Ok(Self::Small),
            "medium" => Ok(Self::Medium),
            "large" => Ok(Self::Large),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1ConsolidationReportRequestCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Micro => write!(f, "micro"),
            Self::Small => write!(f, "small"),
            Self::Medium => write!(f, "medium"),
            Self::Large => write!(f, "large"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
