pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1PurchasesInvoicesCreateResponsePaymentStatus {
    Unpaid,
    Partial,
    Paid,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1PurchasesInvoicesCreateResponsePaymentStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Unpaid => serializer.serialize_str("unpaid"),
            Self::Partial => serializer.serialize_str("partial"),
            Self::Paid => serializer.serialize_str("paid"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1PurchasesInvoicesCreateResponsePaymentStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "unpaid" => Ok(Self::Unpaid),
            "partial" => Ok(Self::Partial),
            "paid" => Ok(Self::Paid),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1PurchasesInvoicesCreateResponsePaymentStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unpaid => write!(f, "unpaid"),
            Self::Partial => write!(f, "partial"),
            Self::Paid => write!(f, "paid"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
