pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1PurchasesInvoicesMatchResponseStatus {
    Matched,
    Mismatched,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1PurchasesInvoicesMatchResponseStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Matched => serializer.serialize_str("matched"),
            Self::Mismatched => serializer.serialize_str("mismatched"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1PurchasesInvoicesMatchResponseStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "matched" => Ok(Self::Matched),
            "mismatched" => Ok(Self::Mismatched),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1PurchasesInvoicesMatchResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Matched => write!(f, "matched"),
            Self::Mismatched => write!(f, "mismatched"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
