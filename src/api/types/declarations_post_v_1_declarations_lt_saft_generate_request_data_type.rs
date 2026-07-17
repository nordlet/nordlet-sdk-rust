pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1DeclarationsLtSaftGenerateRequestDataType {
    F,
    Gl,
    Si,
    Pi,
    Pa,
    Mg,
    As,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1DeclarationsLtSaftGenerateRequestDataType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::F => serializer.serialize_str("F"),
            Self::Gl => serializer.serialize_str("GL"),
            Self::Si => serializer.serialize_str("SI"),
            Self::Pi => serializer.serialize_str("PI"),
            Self::Pa => serializer.serialize_str("PA"),
            Self::Mg => serializer.serialize_str("MG"),
            Self::As => serializer.serialize_str("AS"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1DeclarationsLtSaftGenerateRequestDataType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "F" => Ok(Self::F),
            "GL" => Ok(Self::Gl),
            "SI" => Ok(Self::Si),
            "PI" => Ok(Self::Pi),
            "PA" => Ok(Self::Pa),
            "MG" => Ok(Self::Mg),
            "AS" => Ok(Self::As),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1DeclarationsLtSaftGenerateRequestDataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::F => write!(f, "F"),
            Self::Gl => write!(f, "GL"),
            Self::Si => write!(f, "SI"),
            Self::Pi => write!(f, "PI"),
            Self::Pa => write!(f, "PA"),
            Self::Mg => write!(f, "MG"),
            Self::As => write!(f, "AS"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
