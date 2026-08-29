pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItemMatch {
    Mirrored,
    MatchedByNumber,
    Missing,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItemMatch {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Mirrored => serializer.serialize_str("mirrored"),
            Self::MatchedByNumber => serializer.serialize_str("matched_by_number"),
            Self::Missing => serializer.serialize_str("missing"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de>
    for PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItemMatch
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "mirrored" => Ok(Self::Mirrored),
            "matched_by_number" => Ok(Self::MatchedByNumber),
            "missing" => Ok(Self::Missing),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display
    for PostV1ConsolidationIntercompanyReportResponseDirectionsItemDocumentsItemMatch
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mirrored => write!(f, "mirrored"),
            Self::MatchedByNumber => write!(f, "matched_by_number"),
            Self::Missing => write!(f, "missing"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
