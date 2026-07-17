pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1AccountLocaleSetResponseScope {
    Membership,
    User,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1AccountLocaleSetResponseScope {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Membership => serializer.serialize_str("membership"),
            Self::User => serializer.serialize_str("user"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1AccountLocaleSetResponseScope {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "membership" => Ok(Self::Membership),
            "user" => Ok(Self::User),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1AccountLocaleSetResponseScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Membership => write!(f, "membership"),
            Self::User => write!(f, "user"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
