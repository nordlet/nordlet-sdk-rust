pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VatClassifiersListReferenceResponseRowsItemSide {
    Sale,
    Purchase,
    Both,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for VatClassifiersListReferenceResponseRowsItemSide {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Sale => serializer.serialize_str("sale"),
            Self::Purchase => serializer.serialize_str("purchase"),
            Self::Both => serializer.serialize_str("both"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for VatClassifiersListReferenceResponseRowsItemSide {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "sale" => Ok(Self::Sale),
            "purchase" => Ok(Self::Purchase),
            "both" => Ok(Self::Both),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for VatClassifiersListReferenceResponseRowsItemSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sale => write!(f, "sale"),
            Self::Purchase => write!(f, "purchase"),
            Self::Both => write!(f, "both"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
