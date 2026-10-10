pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetPlatformSellersResponseActivitiesItemPropertyType {
    Dpi901,
    Dpi902,
    Dpi903,
    Dpi904,
    Dpi905,
    Dpi906,
    Dpi907,
    Dpi908,
    Dpi909,
    Dpi910,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetPlatformSellersResponseActivitiesItemPropertyType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Dpi901 => serializer.serialize_str("DPI901"),
            Self::Dpi902 => serializer.serialize_str("DPI902"),
            Self::Dpi903 => serializer.serialize_str("DPI903"),
            Self::Dpi904 => serializer.serialize_str("DPI904"),
            Self::Dpi905 => serializer.serialize_str("DPI905"),
            Self::Dpi906 => serializer.serialize_str("DPI906"),
            Self::Dpi907 => serializer.serialize_str("DPI907"),
            Self::Dpi908 => serializer.serialize_str("DPI908"),
            Self::Dpi909 => serializer.serialize_str("DPI909"),
            Self::Dpi910 => serializer.serialize_str("DPI910"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetPlatformSellersResponseActivitiesItemPropertyType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "DPI901" => Ok(Self::Dpi901),
            "DPI902" => Ok(Self::Dpi902),
            "DPI903" => Ok(Self::Dpi903),
            "DPI904" => Ok(Self::Dpi904),
            "DPI905" => Ok(Self::Dpi905),
            "DPI906" => Ok(Self::Dpi906),
            "DPI907" => Ok(Self::Dpi907),
            "DPI908" => Ok(Self::Dpi908),
            "DPI909" => Ok(Self::Dpi909),
            "DPI910" => Ok(Self::Dpi910),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetPlatformSellersResponseActivitiesItemPropertyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Dpi901 => write!(f, "DPI901"),
            Self::Dpi902 => write!(f, "DPI902"),
            Self::Dpi903 => write!(f, "DPI903"),
            Self::Dpi904 => write!(f, "DPI904"),
            Self::Dpi905 => write!(f, "DPI905"),
            Self::Dpi906 => write!(f, "DPI906"),
            Self::Dpi907 => write!(f, "DPI907"),
            Self::Dpi908 => write!(f, "DPI908"),
            Self::Dpi909 => write!(f, "DPI909"),
            Self::Dpi910 => write!(f, "DPI910"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
