pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1AuditListResponseRowsItemActorType {
    User,
    ApiKey,
    System,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1AuditListResponseRowsItemActorType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::User => serializer.serialize_str("user"),
            Self::ApiKey => serializer.serialize_str("api_key"),
            Self::System => serializer.serialize_str("system"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1AuditListResponseRowsItemActorType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "user" => Ok(Self::User),
            "api_key" => Ok(Self::ApiKey),
            "system" => Ok(Self::System),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1AuditListResponseRowsItemActorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::User => write!(f, "user"),
            Self::ApiKey => write!(f, "api_key"),
            Self::System => write!(f, "system"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
