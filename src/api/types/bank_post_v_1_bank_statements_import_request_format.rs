pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1BankStatementsImportRequestFormat {
    Camt053,
    Mt940,
    StripeCsv,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1BankStatementsImportRequestFormat {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Camt053 => serializer.serialize_str("camt053"),
            Self::Mt940 => serializer.serialize_str("mt940"),
            Self::StripeCsv => serializer.serialize_str("stripe-csv"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1BankStatementsImportRequestFormat {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "camt053" => Ok(Self::Camt053),
            "mt940" => Ok(Self::Mt940),
            "stripe-csv" => Ok(Self::StripeCsv),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1BankStatementsImportRequestFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Camt053 => write!(f, "camt053"),
            Self::Mt940 => write!(f, "mt940"),
            Self::StripeCsv => write!(f, "stripe-csv"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
