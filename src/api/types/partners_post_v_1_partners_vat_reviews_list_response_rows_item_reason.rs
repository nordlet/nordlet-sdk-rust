pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1PartnersVatReviewsListResponseRowsItemReason {
    Invalid,
    ServiceError,
    NameMismatch,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1PartnersVatReviewsListResponseRowsItemReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Invalid => serializer.serialize_str("invalid"),
            Self::ServiceError => serializer.serialize_str("service_error"),
            Self::NameMismatch => serializer.serialize_str("name_mismatch"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1PartnersVatReviewsListResponseRowsItemReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "invalid" => Ok(Self::Invalid),
            "service_error" => Ok(Self::ServiceError),
            "name_mismatch" => Ok(Self::NameMismatch),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1PartnersVatReviewsListResponseRowsItemReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invalid => write!(f, "invalid"),
            Self::ServiceError => write!(f, "service_error"),
            Self::NameMismatch => write!(f, "name_mismatch"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
