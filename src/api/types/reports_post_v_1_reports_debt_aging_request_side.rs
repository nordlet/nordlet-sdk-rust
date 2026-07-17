pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1ReportsDebtAgingRequestSide {
    Receivables,
    Payables,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1ReportsDebtAgingRequestSide {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Receivables => serializer.serialize_str("receivables"),
            Self::Payables => serializer.serialize_str("payables"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1ReportsDebtAgingRequestSide {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "receivables" => Ok(Self::Receivables),
            "payables" => Ok(Self::Payables),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1ReportsDebtAgingRequestSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Receivables => write!(f, "receivables"),
            Self::Payables => write!(f, "payables"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
