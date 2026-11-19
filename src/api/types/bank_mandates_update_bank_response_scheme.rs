pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MandatesUpdateBankResponseScheme {
    Core,
    B2B,
    Bacs,
    Ppd,
    Ccd,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for MandatesUpdateBankResponseScheme {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Core => serializer.serialize_str("CORE"),
            Self::B2B => serializer.serialize_str("B2B"),
            Self::Bacs => serializer.serialize_str("BACS"),
            Self::Ppd => serializer.serialize_str("PPD"),
            Self::Ccd => serializer.serialize_str("CCD"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for MandatesUpdateBankResponseScheme {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "CORE" => Ok(Self::Core),
            "B2B" => Ok(Self::B2B),
            "BACS" => Ok(Self::Bacs),
            "PPD" => Ok(Self::Ppd),
            "CCD" => Ok(Self::Ccd),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for MandatesUpdateBankResponseScheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Core => write!(f, "CORE"),
            Self::B2B => write!(f, "B2B"),
            Self::Bacs => write!(f, "BACS"),
            Self::Ppd => write!(f, "PPD"),
            Self::Ccd => write!(f, "CCD"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
