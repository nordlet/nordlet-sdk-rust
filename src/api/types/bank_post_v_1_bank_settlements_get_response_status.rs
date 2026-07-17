pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1BankSettlementsGetResponseStatus {
    Imported,
    Posted,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1BankSettlementsGetResponseStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Imported => serializer.serialize_str("imported"),
            Self::Posted => serializer.serialize_str("posted"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1BankSettlementsGetResponseStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "imported" => Ok(Self::Imported),
            "posted" => Ok(Self::Posted),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1BankSettlementsGetResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Imported => write!(f, "imported"),
            Self::Posted => write!(f, "posted"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
