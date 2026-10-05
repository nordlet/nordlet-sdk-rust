pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlVatUeGenerateDeclarationsResponseTotalsItemSection {
    C,
    D,
    E,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PlVatUeGenerateDeclarationsResponseTotalsItemSection {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::C => serializer.serialize_str("C"),
            Self::D => serializer.serialize_str("D"),
            Self::E => serializer.serialize_str("E"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PlVatUeGenerateDeclarationsResponseTotalsItemSection {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "C" => Ok(Self::C),
            "D" => Ok(Self::D),
            "E" => Ok(Self::E),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PlVatUeGenerateDeclarationsResponseTotalsItemSection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::C => write!(f, "C"),
            Self::D => write!(f, "D"),
            Self::E => write!(f, "E"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
