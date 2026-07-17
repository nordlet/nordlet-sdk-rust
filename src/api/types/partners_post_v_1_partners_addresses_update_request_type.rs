pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1PartnersAddressesUpdateRequestType {
    Billing,
    Shipping,
    Registered,
    Other,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1PartnersAddressesUpdateRequestType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Billing => serializer.serialize_str("billing"),
            Self::Shipping => serializer.serialize_str("shipping"),
            Self::Registered => serializer.serialize_str("registered"),
            Self::Other => serializer.serialize_str("other"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1PartnersAddressesUpdateRequestType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "billing" => Ok(Self::Billing),
            "shipping" => Ok(Self::Shipping),
            "registered" => Ok(Self::Registered),
            "other" => Ok(Self::Other),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1PartnersAddressesUpdateRequestType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Billing => write!(f, "billing"),
            Self::Shipping => write!(f, "shipping"),
            Self::Registered => write!(f, "registered"),
            Self::Other => write!(f, "other"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
