pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1HrContractsEndResponseType {
    Permanent,
    FixedTerm,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1HrContractsEndResponseType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Permanent => serializer.serialize_str("permanent"),
            Self::FixedTerm => serializer.serialize_str("fixed_term"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1HrContractsEndResponseType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "permanent" => Ok(Self::Permanent),
            "fixed_term" => Ok(Self::FixedTerm),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1HrContractsEndResponseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Permanent => write!(f, "permanent"),
            Self::FixedTerm => write!(f, "fixed_term"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
