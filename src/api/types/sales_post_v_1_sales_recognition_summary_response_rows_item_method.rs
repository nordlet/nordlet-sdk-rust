pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1SalesRecognitionSummaryResponseRowsItemMethod {
    PointInTime,
    Ratable,
    Milestone,
    PercentComplete,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1SalesRecognitionSummaryResponseRowsItemMethod {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::PointInTime => serializer.serialize_str("point_in_time"),
            Self::Ratable => serializer.serialize_str("ratable"),
            Self::Milestone => serializer.serialize_str("milestone"),
            Self::PercentComplete => serializer.serialize_str("percent_complete"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1SalesRecognitionSummaryResponseRowsItemMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "point_in_time" => Ok(Self::PointInTime),
            "ratable" => Ok(Self::Ratable),
            "milestone" => Ok(Self::Milestone),
            "percent_complete" => Ok(Self::PercentComplete),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1SalesRecognitionSummaryResponseRowsItemMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PointInTime => write!(f, "point_in_time"),
            Self::Ratable => write!(f, "ratable"),
            Self::Milestone => write!(f, "milestone"),
            Self::PercentComplete => write!(f, "percent_complete"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
