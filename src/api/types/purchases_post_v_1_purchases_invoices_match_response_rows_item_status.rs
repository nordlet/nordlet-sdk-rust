pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1PurchasesInvoicesMatchResponseRowsItemStatus {
    Matched,
    NotReceived,
    OverInvoiced,
    PriceMismatch,
    NotOnOrder,
    NotInvoiced,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1PurchasesInvoicesMatchResponseRowsItemStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Matched => serializer.serialize_str("matched"),
            Self::NotReceived => serializer.serialize_str("not_received"),
            Self::OverInvoiced => serializer.serialize_str("over_invoiced"),
            Self::PriceMismatch => serializer.serialize_str("price_mismatch"),
            Self::NotOnOrder => serializer.serialize_str("not_on_order"),
            Self::NotInvoiced => serializer.serialize_str("not_invoiced"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1PurchasesInvoicesMatchResponseRowsItemStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "matched" => Ok(Self::Matched),
            "not_received" => Ok(Self::NotReceived),
            "over_invoiced" => Ok(Self::OverInvoiced),
            "price_mismatch" => Ok(Self::PriceMismatch),
            "not_on_order" => Ok(Self::NotOnOrder),
            "not_invoiced" => Ok(Self::NotInvoiced),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1PurchasesInvoicesMatchResponseRowsItemStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Matched => write!(f, "matched"),
            Self::NotReceived => write!(f, "not_received"),
            Self::OverInvoiced => write!(f, "over_invoiced"),
            Self::PriceMismatch => write!(f, "price_mismatch"),
            Self::NotOnOrder => write!(f, "not_on_order"),
            Self::NotInvoiced => write!(f, "not_invoiced"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
