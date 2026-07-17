pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1ProductionOrdersListResponseRowsItemType {
    Assembly,
    Disassembly,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1ProductionOrdersListResponseRowsItemType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Assembly => serializer.serialize_str("assembly"),
            Self::Disassembly => serializer.serialize_str("disassembly"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1ProductionOrdersListResponseRowsItemType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "assembly" => Ok(Self::Assembly),
            "disassembly" => Ok(Self::Disassembly),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1ProductionOrdersListResponseRowsItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Assembly => write!(f, "assembly"),
            Self::Disassembly => write!(f, "disassembly"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
