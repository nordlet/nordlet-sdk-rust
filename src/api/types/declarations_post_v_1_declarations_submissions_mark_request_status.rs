pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1DeclarationsSubmissionsMarkRequestStatus {
    Submitted,
    Accepted,
    Rejected,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1DeclarationsSubmissionsMarkRequestStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Submitted => serializer.serialize_str("submitted"),
            Self::Accepted => serializer.serialize_str("accepted"),
            Self::Rejected => serializer.serialize_str("rejected"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1DeclarationsSubmissionsMarkRequestStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "submitted" => Ok(Self::Submitted),
            "accepted" => Ok(Self::Accepted),
            "rejected" => Ok(Self::Rejected),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1DeclarationsSubmissionsMarkRequestStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Submitted => write!(f, "submitted"),
            Self::Accepted => write!(f, "accepted"),
            Self::Rejected => write!(f, "rejected"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
