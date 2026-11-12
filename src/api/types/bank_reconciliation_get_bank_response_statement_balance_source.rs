pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReconciliationGetBankResponseStatementBalanceSource {
    Request,
    Statement,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ReconciliationGetBankResponseStatementBalanceSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Request => serializer.serialize_str("request"),
            Self::Statement => serializer.serialize_str("statement"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ReconciliationGetBankResponseStatementBalanceSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "request" => Ok(Self::Request),
            "statement" => Ok(Self::Statement),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ReconciliationGetBankResponseStatementBalanceSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Request => write!(f, "request"),
            Self::Statement => write!(f, "statement"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
