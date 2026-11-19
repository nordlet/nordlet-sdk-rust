pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CompaniesProfileAccountResponseFiscalYearWeekRule {
    Last,
    Nearest,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CompaniesProfileAccountResponseFiscalYearWeekRule {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Last => serializer.serialize_str("last"),
            Self::Nearest => serializer.serialize_str("nearest"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CompaniesProfileAccountResponseFiscalYearWeekRule {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "last" => Ok(Self::Last),
            "nearest" => Ok(Self::Nearest),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CompaniesProfileAccountResponseFiscalYearWeekRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Last => write!(f, "last"),
            Self::Nearest => write!(f, "nearest"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
