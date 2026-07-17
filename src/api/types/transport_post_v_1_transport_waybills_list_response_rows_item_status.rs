pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1TransportWaybillsListResponseRowsItemStatus {
    Draft,
    Issued,
    Cancelled,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1TransportWaybillsListResponseRowsItemStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Draft => serializer.serialize_str("draft"),
            Self::Issued => serializer.serialize_str("issued"),
            Self::Cancelled => serializer.serialize_str("cancelled"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1TransportWaybillsListResponseRowsItemStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "draft" => Ok(Self::Draft),
            "issued" => Ok(Self::Issued),
            "cancelled" => Ok(Self::Cancelled),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1TransportWaybillsListResponseRowsItemStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Draft => write!(f, "draft"),
            Self::Issued => write!(f, "issued"),
            Self::Cancelled => write!(f, "cancelled"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
