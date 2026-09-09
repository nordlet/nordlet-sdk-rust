pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1LeadsGetResponseStatus {
    New,
    Contacted,
    Qualified,
    Lost,
    Converted,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1LeadsGetResponseStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::New => serializer.serialize_str("new"),
            Self::Contacted => serializer.serialize_str("contacted"),
            Self::Qualified => serializer.serialize_str("qualified"),
            Self::Lost => serializer.serialize_str("lost"),
            Self::Converted => serializer.serialize_str("converted"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1LeadsGetResponseStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "new" => Ok(Self::New),
            "contacted" => Ok(Self::Contacted),
            "qualified" => Ok(Self::Qualified),
            "lost" => Ok(Self::Lost),
            "converted" => Ok(Self::Converted),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1LeadsGetResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::New => write!(f, "new"),
            Self::Contacted => write!(f, "contacted"),
            Self::Qualified => write!(f, "qualified"),
            Self::Lost => write!(f, "lost"),
            Self::Converted => write!(f, "converted"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
