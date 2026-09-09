pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1PartnersFindOrCreateResponsePartnerLegalCountryClass {
    Lt,
    Eu,
    NonEu,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1PartnersFindOrCreateResponsePartnerLegalCountryClass {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Lt => serializer.serialize_str("lt"),
            Self::Eu => serializer.serialize_str("eu"),
            Self::NonEu => serializer.serialize_str("non_eu"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1PartnersFindOrCreateResponsePartnerLegalCountryClass {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "lt" => Ok(Self::Lt),
            "eu" => Ok(Self::Eu),
            "non_eu" => Ok(Self::NonEu),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1PartnersFindOrCreateResponsePartnerLegalCountryClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Lt => write!(f, "lt"),
            Self::Eu => write!(f, "eu"),
            Self::NonEu => write!(f, "non_eu"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
