pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1AgreementsAgreementsGetResponseStatus {
    Draft,
    Active,
    Expired,
    Terminated,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1AgreementsAgreementsGetResponseStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Draft => serializer.serialize_str("draft"),
            Self::Active => serializer.serialize_str("active"),
            Self::Expired => serializer.serialize_str("expired"),
            Self::Terminated => serializer.serialize_str("terminated"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1AgreementsAgreementsGetResponseStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "draft" => Ok(Self::Draft),
            "active" => Ok(Self::Active),
            "expired" => Ok(Self::Expired),
            "terminated" => Ok(Self::Terminated),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1AgreementsAgreementsGetResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Draft => write!(f, "draft"),
            Self::Active => write!(f, "active"),
            Self::Expired => write!(f, "expired"),
            Self::Terminated => write!(f, "terminated"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
