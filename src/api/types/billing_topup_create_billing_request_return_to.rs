pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TopupCreateBillingRequestReturnTo {
    Credits,
    Onboarding,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TopupCreateBillingRequestReturnTo {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Credits => serializer.serialize_str("credits"),
            Self::Onboarding => serializer.serialize_str("onboarding"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TopupCreateBillingRequestReturnTo {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "credits" => Ok(Self::Credits),
            "onboarding" => Ok(Self::Onboarding),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TopupCreateBillingRequestReturnTo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Credits => write!(f, "credits"),
            Self::Onboarding => write!(f, "onboarding"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
