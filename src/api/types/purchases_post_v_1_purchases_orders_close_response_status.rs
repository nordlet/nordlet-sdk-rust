pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1PurchasesOrdersCloseResponseStatus {
    Draft,
    Submitted,
    Approved,
    PartiallyReceived,
    Received,
    Closed,
    Cancelled,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1PurchasesOrdersCloseResponseStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Draft => serializer.serialize_str("draft"),
            Self::Submitted => serializer.serialize_str("submitted"),
            Self::Approved => serializer.serialize_str("approved"),
            Self::PartiallyReceived => serializer.serialize_str("partially_received"),
            Self::Received => serializer.serialize_str("received"),
            Self::Closed => serializer.serialize_str("closed"),
            Self::Cancelled => serializer.serialize_str("cancelled"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1PurchasesOrdersCloseResponseStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "draft" => Ok(Self::Draft),
            "submitted" => Ok(Self::Submitted),
            "approved" => Ok(Self::Approved),
            "partially_received" => Ok(Self::PartiallyReceived),
            "received" => Ok(Self::Received),
            "closed" => Ok(Self::Closed),
            "cancelled" => Ok(Self::Cancelled),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1PurchasesOrdersCloseResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Draft => write!(f, "draft"),
            Self::Submitted => write!(f, "submitted"),
            Self::Approved => write!(f, "approved"),
            Self::PartiallyReceived => write!(f, "partially_received"),
            Self::Received => write!(f, "received"),
            Self::Closed => write!(f, "closed"),
            Self::Cancelled => write!(f, "cancelled"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
