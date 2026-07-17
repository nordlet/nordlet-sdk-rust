pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1PurchasesInvoicesCreateResponseType {
    Invoice,
    CreditNote,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1PurchasesInvoicesCreateResponseType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Invoice => serializer.serialize_str("invoice"),
            Self::CreditNote => serializer.serialize_str("credit_note"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1PurchasesInvoicesCreateResponseType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "invoice" => Ok(Self::Invoice),
            "credit_note" => Ok(Self::CreditNote),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1PurchasesInvoicesCreateResponseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invoice => write!(f, "invoice"),
            Self::CreditNote => write!(f, "credit_note"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
