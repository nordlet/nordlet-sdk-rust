pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1BillingAccountSetPlanResponsePlan {
    Starter,
    Business,
    Scale,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1BillingAccountSetPlanResponsePlan {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Starter => serializer.serialize_str("starter"),
            Self::Business => serializer.serialize_str("business"),
            Self::Scale => serializer.serialize_str("scale"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1BillingAccountSetPlanResponsePlan {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "starter" => Ok(Self::Starter),
            "business" => Ok(Self::Business),
            "scale" => Ok(Self::Scale),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1BillingAccountSetPlanResponsePlan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Starter => write!(f, "starter"),
            Self::Business => write!(f, "business"),
            Self::Scale => write!(f, "scale"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
