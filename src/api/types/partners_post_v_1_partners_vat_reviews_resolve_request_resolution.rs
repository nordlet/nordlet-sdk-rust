pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1PartnersVatReviewsResolveRequestResolution {
    ConfirmedValid,
    ConfirmedInvalid,
    Dismissed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1PartnersVatReviewsResolveRequestResolution {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ConfirmedValid => serializer.serialize_str("confirmed_valid"),
            Self::ConfirmedInvalid => serializer.serialize_str("confirmed_invalid"),
            Self::Dismissed => serializer.serialize_str("dismissed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1PartnersVatReviewsResolveRequestResolution {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "confirmed_valid" => Ok(Self::ConfirmedValid),
            "confirmed_invalid" => Ok(Self::ConfirmedInvalid),
            "dismissed" => Ok(Self::Dismissed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1PartnersVatReviewsResolveRequestResolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConfirmedValid => write!(f, "confirmed_valid"),
            Self::ConfirmedInvalid => write!(f, "confirmed_invalid"),
            Self::Dismissed => write!(f, "dismissed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
