pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AccountsCreateBankRequestType {
    Bank,
    Stripe,
    Broker,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AccountsCreateBankRequestType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Bank => serializer.serialize_str("bank"),
            Self::Stripe => serializer.serialize_str("stripe"),
            Self::Broker => serializer.serialize_str("broker"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AccountsCreateBankRequestType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "bank" => Ok(Self::Bank),
            "stripe" => Ok(Self::Stripe),
            "broker" => Ok(Self::Broker),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AccountsCreateBankRequestType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bank => write!(f, "bank"),
            Self::Stripe => write!(f, "stripe"),
            Self::Broker => write!(f, "broker"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
