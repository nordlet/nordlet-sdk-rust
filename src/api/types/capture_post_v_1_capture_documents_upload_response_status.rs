pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1CaptureDocumentsUploadResponseStatus {
    Pending,
    Extracted,
    Failed,
    Linked,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1CaptureDocumentsUploadResponseStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Pending => serializer.serialize_str("pending"),
            Self::Extracted => serializer.serialize_str("extracted"),
            Self::Failed => serializer.serialize_str("failed"),
            Self::Linked => serializer.serialize_str("linked"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1CaptureDocumentsUploadResponseStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "pending" => Ok(Self::Pending),
            "extracted" => Ok(Self::Extracted),
            "failed" => Ok(Self::Failed),
            "linked" => Ok(Self::Linked),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1CaptureDocumentsUploadResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Extracted => write!(f, "extracted"),
            Self::Failed => write!(f, "failed"),
            Self::Linked => write!(f, "linked"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
