pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InvitesCreateAccountRequestLocale {
    En,
    Lt,
    De,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for InvitesCreateAccountRequestLocale {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::En => serializer.serialize_str("en"),
            Self::Lt => serializer.serialize_str("lt"),
            Self::De => serializer.serialize_str("de"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for InvitesCreateAccountRequestLocale {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "en" => Ok(Self::En),
            "lt" => Ok(Self::Lt),
            "de" => Ok(Self::De),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for InvitesCreateAccountRequestLocale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::En => write!(f, "en"),
            Self::Lt => write!(f, "lt"),
            Self::De => write!(f, "de"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
