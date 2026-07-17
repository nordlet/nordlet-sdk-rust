pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1HrContractsCreateResponseSalaryType {
    Monthly,
    Hourly,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1HrContractsCreateResponseSalaryType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Monthly => serializer.serialize_str("monthly"),
            Self::Hourly => serializer.serialize_str("hourly"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1HrContractsCreateResponseSalaryType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "monthly" => Ok(Self::Monthly),
            "hourly" => Ok(Self::Hourly),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1HrContractsCreateResponseSalaryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Monthly => write!(f, "monthly"),
            Self::Hourly => write!(f, "hourly"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
