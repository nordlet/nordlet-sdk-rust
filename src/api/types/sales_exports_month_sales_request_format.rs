pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExportsMonthSalesRequestFormat {
    Csv,
    Pdf,
    Nordlet,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ExportsMonthSalesRequestFormat {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Csv => serializer.serialize_str("csv"),
            Self::Pdf => serializer.serialize_str("pdf"),
            Self::Nordlet => serializer.serialize_str("nordlet"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ExportsMonthSalesRequestFormat {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "csv" => Ok(Self::Csv),
            "pdf" => Ok(Self::Pdf),
            "nordlet" => Ok(Self::Nordlet),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ExportsMonthSalesRequestFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Csv => write!(f, "csv"),
            Self::Pdf => write!(f, "pdf"),
            Self::Nordlet => write!(f, "nordlet"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
