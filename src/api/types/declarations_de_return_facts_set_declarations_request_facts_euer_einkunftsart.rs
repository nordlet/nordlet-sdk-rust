pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DeReturnFactsSetDeclarationsRequestFactsEuerEinkunftsart {
    One,
    Two,
    Three,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DeReturnFactsSetDeclarationsRequestFactsEuerEinkunftsart {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::One => serializer.serialize_str("1"),
            Self::Two => serializer.serialize_str("2"),
            Self::Three => serializer.serialize_str("3"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DeReturnFactsSetDeclarationsRequestFactsEuerEinkunftsart {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "1" => Ok(Self::One),
            "2" => Ok(Self::Two),
            "3" => Ok(Self::Three),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DeReturnFactsSetDeclarationsRequestFactsEuerEinkunftsart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::One => write!(f, "1"),
            Self::Two => write!(f, "2"),
            Self::Three => write!(f, "3"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
