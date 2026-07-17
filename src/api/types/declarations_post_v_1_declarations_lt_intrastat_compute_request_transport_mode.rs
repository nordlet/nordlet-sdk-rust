pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1DeclarationsLtIntrastatComputeRequestTransportMode {
    One,
    Two,
    Three,
    Four,
    Five,
    Seven,
    Eight,
    Nine,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1DeclarationsLtIntrastatComputeRequestTransportMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::One => serializer.serialize_str("1"),
            Self::Two => serializer.serialize_str("2"),
            Self::Three => serializer.serialize_str("3"),
            Self::Four => serializer.serialize_str("4"),
            Self::Five => serializer.serialize_str("5"),
            Self::Seven => serializer.serialize_str("7"),
            Self::Eight => serializer.serialize_str("8"),
            Self::Nine => serializer.serialize_str("9"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1DeclarationsLtIntrastatComputeRequestTransportMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "1" => Ok(Self::One),
            "2" => Ok(Self::Two),
            "3" => Ok(Self::Three),
            "4" => Ok(Self::Four),
            "5" => Ok(Self::Five),
            "7" => Ok(Self::Seven),
            "8" => Ok(Self::Eight),
            "9" => Ok(Self::Nine),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1DeclarationsLtIntrastatComputeRequestTransportMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::One => write!(f, "1"),
            Self::Two => write!(f, "2"),
            Self::Three => write!(f, "3"),
            Self::Four => write!(f, "4"),
            Self::Five => write!(f, "5"),
            Self::Seven => write!(f, "7"),
            Self::Eight => write!(f, "8"),
            Self::Nine => write!(f, "9"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
