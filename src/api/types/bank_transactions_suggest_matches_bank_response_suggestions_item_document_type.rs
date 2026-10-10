pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TransactionsSuggestMatchesBankResponseSuggestionsItemDocumentType {
    SaleInvoice,
    PurchaseInvoice,
    PayrollRun,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TransactionsSuggestMatchesBankResponseSuggestionsItemDocumentType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::SaleInvoice => serializer.serialize_str("sale_invoice"),
            Self::PurchaseInvoice => serializer.serialize_str("purchase_invoice"),
            Self::PayrollRun => serializer.serialize_str("payroll_run"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TransactionsSuggestMatchesBankResponseSuggestionsItemDocumentType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "sale_invoice" => Ok(Self::SaleInvoice),
            "purchase_invoice" => Ok(Self::PurchaseInvoice),
            "payroll_run" => Ok(Self::PayrollRun),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TransactionsSuggestMatchesBankResponseSuggestionsItemDocumentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SaleInvoice => write!(f, "sale_invoice"),
            Self::PurchaseInvoice => write!(f, "purchase_invoice"),
            Self::PayrollRun => write!(f, "payroll_run"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
