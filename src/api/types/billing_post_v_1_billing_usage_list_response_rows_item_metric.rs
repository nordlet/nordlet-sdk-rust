pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1BillingUsageListResponseRowsItemMetric {
    ApiRequest,
    OcrPage,
    FileStorageBytes,
    DatabaseBytes,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1BillingUsageListResponseRowsItemMetric {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ApiRequest => serializer.serialize_str("api_request"),
            Self::OcrPage => serializer.serialize_str("ocr_page"),
            Self::FileStorageBytes => serializer.serialize_str("file_storage_bytes"),
            Self::DatabaseBytes => serializer.serialize_str("database_bytes"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1BillingUsageListResponseRowsItemMetric {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "api_request" => Ok(Self::ApiRequest),
            "ocr_page" => Ok(Self::OcrPage),
            "file_storage_bytes" => Ok(Self::FileStorageBytes),
            "database_bytes" => Ok(Self::DatabaseBytes),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1BillingUsageListResponseRowsItemMetric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ApiRequest => write!(f, "api_request"),
            Self::OcrPage => write!(f, "ocr_page"),
            Self::FileStorageBytes => write!(f, "file_storage_bytes"),
            Self::DatabaseBytes => write!(f, "database_bytes"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
