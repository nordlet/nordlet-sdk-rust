pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1BankImportTemplatesCreateRequestType {
    Stripe,
    Iso20022,
    BankConnection,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1BankImportTemplatesCreateRequestType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Stripe => serializer.serialize_str("stripe"),
            Self::Iso20022 => serializer.serialize_str("iso20022"),
            Self::BankConnection => serializer.serialize_str("bank_connection"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1BankImportTemplatesCreateRequestType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "stripe" => Ok(Self::Stripe),
            "iso20022" => Ok(Self::Iso20022),
            "bank_connection" => Ok(Self::BankConnection),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1BankImportTemplatesCreateRequestType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Stripe => write!(f, "stripe"),
            Self::Iso20022 => write!(f, "iso20022"),
            Self::BankConnection => write!(f, "bank_connection"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
