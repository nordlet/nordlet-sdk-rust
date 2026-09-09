pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1DocumentSeriesCreateRequestDocumentType {
    SaleInvoice,
    SaleCreditNote,
    SaleProforma,
    SaleAdvance,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1DocumentSeriesCreateRequestDocumentType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::SaleInvoice => serializer.serialize_str("sale_invoice"),
            Self::SaleCreditNote => serializer.serialize_str("sale_credit_note"),
            Self::SaleProforma => serializer.serialize_str("sale_proforma"),
            Self::SaleAdvance => serializer.serialize_str("sale_advance"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1DocumentSeriesCreateRequestDocumentType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "sale_invoice" => Ok(Self::SaleInvoice),
            "sale_credit_note" => Ok(Self::SaleCreditNote),
            "sale_proforma" => Ok(Self::SaleProforma),
            "sale_advance" => Ok(Self::SaleAdvance),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1DocumentSeriesCreateRequestDocumentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SaleInvoice => write!(f, "sale_invoice"),
            Self::SaleCreditNote => write!(f, "sale_credit_note"),
            Self::SaleProforma => write!(f, "sale_proforma"),
            Self::SaleAdvance => write!(f, "sale_advance"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
