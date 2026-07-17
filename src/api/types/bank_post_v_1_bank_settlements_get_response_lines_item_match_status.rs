pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1BankSettlementsGetResponseLinesItemMatchStatus {
    Unmatched,
    Matched,
    Manual,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1BankSettlementsGetResponseLinesItemMatchStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Unmatched => serializer.serialize_str("unmatched"),
            Self::Matched => serializer.serialize_str("matched"),
            Self::Manual => serializer.serialize_str("manual"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1BankSettlementsGetResponseLinesItemMatchStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "unmatched" => Ok(Self::Unmatched),
            "matched" => Ok(Self::Matched),
            "manual" => Ok(Self::Manual),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1BankSettlementsGetResponseLinesItemMatchStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unmatched => write!(f, "unmatched"),
            Self::Matched => write!(f, "matched"),
            Self::Manual => write!(f, "manual"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
