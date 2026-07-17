pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1PartnersInquiriesListResponseRowsItemStatus {
    New,
    InProgress,
    Closed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1PartnersInquiriesListResponseRowsItemStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::New => serializer.serialize_str("new"),
            Self::InProgress => serializer.serialize_str("in_progress"),
            Self::Closed => serializer.serialize_str("closed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1PartnersInquiriesListResponseRowsItemStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "new" => Ok(Self::New),
            "in_progress" => Ok(Self::InProgress),
            "closed" => Ok(Self::Closed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1PartnersInquiriesListResponseRowsItemStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::New => write!(f, "new"),
            Self::InProgress => write!(f, "in_progress"),
            Self::Closed => write!(f, "closed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
