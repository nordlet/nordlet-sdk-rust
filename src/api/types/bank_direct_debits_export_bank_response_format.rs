pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DirectDebitsExportBankResponseFormat {
    Pain00100103,
    Pain00800102,
    BacsStandard18,
    Nacha,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DirectDebitsExportBankResponseFormat {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Pain00100103 => serializer.serialize_str("pain.001.001.03"),
            Self::Pain00800102 => serializer.serialize_str("pain.008.001.02"),
            Self::BacsStandard18 => serializer.serialize_str("bacs-standard-18"),
            Self::Nacha => serializer.serialize_str("nacha"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DirectDebitsExportBankResponseFormat {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "pain.001.001.03" => Ok(Self::Pain00100103),
            "pain.008.001.02" => Ok(Self::Pain00800102),
            "bacs-standard-18" => Ok(Self::BacsStandard18),
            "nacha" => Ok(Self::Nacha),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DirectDebitsExportBankResponseFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pain00100103 => write!(f, "pain.001.001.03"),
            Self::Pain00800102 => write!(f, "pain.008.001.02"),
            Self::BacsStandard18 => write!(f, "bacs-standard-18"),
            Self::Nacha => write!(f, "nacha"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
