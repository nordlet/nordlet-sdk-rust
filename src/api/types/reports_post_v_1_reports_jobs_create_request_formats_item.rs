pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1ReportsJobsCreateRequestFormatsItem {
    Json,
    Xlsx,
    Pdf,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1ReportsJobsCreateRequestFormatsItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Json => serializer.serialize_str("json"),
            Self::Xlsx => serializer.serialize_str("xlsx"),
            Self::Pdf => serializer.serialize_str("pdf"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1ReportsJobsCreateRequestFormatsItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "json" => Ok(Self::Json),
            "xlsx" => Ok(Self::Xlsx),
            "pdf" => Ok(Self::Pdf),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1ReportsJobsCreateRequestFormatsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json => write!(f, "json"),
            Self::Xlsx => write!(f, "xlsx"),
            Self::Pdf => write!(f, "pdf"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
