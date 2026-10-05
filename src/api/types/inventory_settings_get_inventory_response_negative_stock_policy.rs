pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SettingsGetInventoryResponseNegativeStockPolicy {
    Reject,
    Allow,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SettingsGetInventoryResponseNegativeStockPolicy {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Reject => serializer.serialize_str("reject"),
            Self::Allow => serializer.serialize_str("allow"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SettingsGetInventoryResponseNegativeStockPolicy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "reject" => Ok(Self::Reject),
            "allow" => Ok(Self::Allow),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SettingsGetInventoryResponseNegativeStockPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Reject => write!(f, "reject"),
            Self::Allow => write!(f, "allow"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
