pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TimeEntriesBillProjectsRequestGroupBy {
    Rate,
    Entry,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TimeEntriesBillProjectsRequestGroupBy {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Rate => serializer.serialize_str("rate"),
            Self::Entry => serializer.serialize_str("entry"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TimeEntriesBillProjectsRequestGroupBy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "rate" => Ok(Self::Rate),
            "entry" => Ok(Self::Entry),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TimeEntriesBillProjectsRequestGroupBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rate => write!(f, "rate"),
            Self::Entry => write!(f, "entry"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
