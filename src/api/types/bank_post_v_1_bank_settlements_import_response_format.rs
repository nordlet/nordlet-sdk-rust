pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1BankSettlementsImportResponseFormat {
    PayoutReconciliation,
    UnifiedPayments,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1BankSettlementsImportResponseFormat {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::PayoutReconciliation => serializer.serialize_str("payout_reconciliation"),
            Self::UnifiedPayments => serializer.serialize_str("unified_payments"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1BankSettlementsImportResponseFormat {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "payout_reconciliation" => Ok(Self::PayoutReconciliation),
            "unified_payments" => Ok(Self::UnifiedPayments),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1BankSettlementsImportResponseFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PayoutReconciliation => write!(f, "payout_reconciliation"),
            Self::UnifiedPayments => write!(f, "unified_payments"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
