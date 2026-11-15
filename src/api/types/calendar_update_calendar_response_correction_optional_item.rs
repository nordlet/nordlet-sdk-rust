pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateCalendarResponseCorrectionOptionalItem {
    Reason,
    ReasonCode,
    AuthorityReference,
    DiscoveredOn,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UpdateCalendarResponseCorrectionOptionalItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Reason => serializer.serialize_str("reason"),
            Self::ReasonCode => serializer.serialize_str("reasonCode"),
            Self::AuthorityReference => serializer.serialize_str("authorityReference"),
            Self::DiscoveredOn => serializer.serialize_str("discoveredOn"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UpdateCalendarResponseCorrectionOptionalItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "reason" => Ok(Self::Reason),
            "reasonCode" => Ok(Self::ReasonCode),
            "authorityReference" => Ok(Self::AuthorityReference),
            "discoveredOn" => Ok(Self::DiscoveredOn),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UpdateCalendarResponseCorrectionOptionalItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Reason => write!(f, "reason"),
            Self::ReasonCode => write!(f, "reasonCode"),
            Self::AuthorityReference => write!(f, "authorityReference"),
            Self::DiscoveredOn => write!(f, "discoveredOn"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
