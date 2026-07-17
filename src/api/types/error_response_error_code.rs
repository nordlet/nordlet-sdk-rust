pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ErrorResponseErrorCode {
    Validation,
    Unauthorized,
    Forbidden,
    NotFound,
    Conflict,
    IdempotencyKeyReuse,
    IdempotencyInProgress,
    RateLimited,
    Internal,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ErrorResponseErrorCode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Validation => serializer.serialize_str("validation"),
            Self::Unauthorized => serializer.serialize_str("unauthorized"),
            Self::Forbidden => serializer.serialize_str("forbidden"),
            Self::NotFound => serializer.serialize_str("not_found"),
            Self::Conflict => serializer.serialize_str("conflict"),
            Self::IdempotencyKeyReuse => serializer.serialize_str("idempotency_key_reuse"),
            Self::IdempotencyInProgress => serializer.serialize_str("idempotency_in_progress"),
            Self::RateLimited => serializer.serialize_str("rate_limited"),
            Self::Internal => serializer.serialize_str("internal"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ErrorResponseErrorCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "validation" => Ok(Self::Validation),
            "unauthorized" => Ok(Self::Unauthorized),
            "forbidden" => Ok(Self::Forbidden),
            "not_found" => Ok(Self::NotFound),
            "conflict" => Ok(Self::Conflict),
            "idempotency_key_reuse" => Ok(Self::IdempotencyKeyReuse),
            "idempotency_in_progress" => Ok(Self::IdempotencyInProgress),
            "rate_limited" => Ok(Self::RateLimited),
            "internal" => Ok(Self::Internal),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ErrorResponseErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Validation => write!(f, "validation"),
            Self::Unauthorized => write!(f, "unauthorized"),
            Self::Forbidden => write!(f, "forbidden"),
            Self::NotFound => write!(f, "not_found"),
            Self::Conflict => write!(f, "conflict"),
            Self::IdempotencyKeyReuse => write!(f, "idempotency_key_reuse"),
            Self::IdempotencyInProgress => write!(f, "idempotency_in_progress"),
            Self::RateLimited => write!(f, "rate_limited"),
            Self::Internal => write!(f, "internal"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
