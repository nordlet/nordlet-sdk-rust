pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CertificatesDeleteDeclarationsResponseRowsItemFormat {
    Pem,
    PemKey,
    Pfx,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CertificatesDeleteDeclarationsResponseRowsItemFormat {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Pem => serializer.serialize_str("pem"),
            Self::PemKey => serializer.serialize_str("pem-key"),
            Self::Pfx => serializer.serialize_str("pfx"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CertificatesDeleteDeclarationsResponseRowsItemFormat {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "pem" => Ok(Self::Pem),
            "pem-key" => Ok(Self::PemKey),
            "pfx" => Ok(Self::Pfx),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CertificatesDeleteDeclarationsResponseRowsItemFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pem => write!(f, "pem"),
            Self::PemKey => write!(f, "pem-key"),
            Self::Pfx => write!(f, "pfx"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
