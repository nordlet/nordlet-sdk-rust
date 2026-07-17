pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1BankTransactionsMatchResponseStatus {
    New,
    Matched,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1BankTransactionsMatchResponseStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::New => serializer.serialize_str("new"),
            Self::Matched => serializer.serialize_str("matched"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1BankTransactionsMatchResponseStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "new" => Ok(Self::New),
            "matched" => Ok(Self::Matched),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1BankTransactionsMatchResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::New => write!(f, "new"),
            Self::Matched => write!(f, "matched"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
