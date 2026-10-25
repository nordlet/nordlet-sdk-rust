pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PaymentsListSalesResponseRowsItemMethod {
    BankTransfer,
    Card,
    Cash,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PaymentsListSalesResponseRowsItemMethod {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::BankTransfer => serializer.serialize_str("bank_transfer"),
            Self::Card => serializer.serialize_str("card"),
            Self::Cash => serializer.serialize_str("cash"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PaymentsListSalesResponseRowsItemMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "bank_transfer" => Ok(Self::BankTransfer),
            "card" => Ok(Self::Card),
            "cash" => Ok(Self::Cash),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PaymentsListSalesResponseRowsItemMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BankTransfer => write!(f, "bank_transfer"),
            Self::Card => write!(f, "card"),
            Self::Cash => write!(f, "cash"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
