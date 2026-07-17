pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1ConsolidationReportResponseIntercompanyCandidatesItemMatchedOn {
    Code,
    VatCode,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1ConsolidationReportResponseIntercompanyCandidatesItemMatchedOn {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Code => serializer.serialize_str("code"),
            Self::VatCode => serializer.serialize_str("vatCode"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de>
    for PostV1ConsolidationReportResponseIntercompanyCandidatesItemMatchedOn
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "code" => Ok(Self::Code),
            "vatCode" => Ok(Self::VatCode),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1ConsolidationReportResponseIntercompanyCandidatesItemMatchedOn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Code => write!(f, "code"),
            Self::VatCode => write!(f, "vatCode"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
