pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EuDigitalReportingListDeclarationsResponseTransactionsItemDirection {
    Supply,
    Acquisition,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for EuDigitalReportingListDeclarationsResponseTransactionsItemDirection {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Supply => serializer.serialize_str("supply"),
            Self::Acquisition => serializer.serialize_str("acquisition"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for EuDigitalReportingListDeclarationsResponseTransactionsItemDirection {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "supply" => Ok(Self::Supply),
            "acquisition" => Ok(Self::Acquisition),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for EuDigitalReportingListDeclarationsResponseTransactionsItemDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Supply => write!(f, "supply"),
            Self::Acquisition => write!(f, "acquisition"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
