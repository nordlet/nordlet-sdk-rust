pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1DeclarationsCertificatesDeleteRequestFieldKey {
    Certificate,
    PrivateKey,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1DeclarationsCertificatesDeleteRequestFieldKey {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Certificate => serializer.serialize_str("certificate"),
            Self::PrivateKey => serializer.serialize_str("privateKey"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1DeclarationsCertificatesDeleteRequestFieldKey {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "certificate" => Ok(Self::Certificate),
            "privateKey" => Ok(Self::PrivateKey),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1DeclarationsCertificatesDeleteRequestFieldKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Certificate => write!(f, "certificate"),
            Self::PrivateKey => write!(f, "privateKey"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
