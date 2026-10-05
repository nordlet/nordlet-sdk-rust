pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConfigsUpdateDeclarationsResponseFieldsItemKind {
    Text,
    Secret,
    Select,
    Url,
    Certificate,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ConfigsUpdateDeclarationsResponseFieldsItemKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Text => serializer.serialize_str("text"),
            Self::Secret => serializer.serialize_str("secret"),
            Self::Select => serializer.serialize_str("select"),
            Self::Url => serializer.serialize_str("url"),
            Self::Certificate => serializer.serialize_str("certificate"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ConfigsUpdateDeclarationsResponseFieldsItemKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "text" => Ok(Self::Text),
            "secret" => Ok(Self::Secret),
            "select" => Ok(Self::Select),
            "url" => Ok(Self::Url),
            "certificate" => Ok(Self::Certificate),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ConfigsUpdateDeclarationsResponseFieldsItemKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text => write!(f, "text"),
            Self::Secret => write!(f, "secret"),
            Self::Select => write!(f, "select"),
            Self::Url => write!(f, "url"),
            Self::Certificate => write!(f, "certificate"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
