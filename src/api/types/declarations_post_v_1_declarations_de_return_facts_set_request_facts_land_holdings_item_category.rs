pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1DeclarationsDeReturnFactsSetRequestFactsLandHoldingsItemCategory {
    RentalEast,
    BusinessEast,
    MixedEast,
    UndevelopedEast,
    Other,
    Agricultural,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1DeclarationsDeReturnFactsSetRequestFactsLandHoldingsItemCategory {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::RentalEast => serializer.serialize_str("rental_east"),
            Self::BusinessEast => serializer.serialize_str("business_east"),
            Self::MixedEast => serializer.serialize_str("mixed_east"),
            Self::UndevelopedEast => serializer.serialize_str("undeveloped_east"),
            Self::Other => serializer.serialize_str("other"),
            Self::Agricultural => serializer.serialize_str("agricultural"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de>
    for PostV1DeclarationsDeReturnFactsSetRequestFactsLandHoldingsItemCategory
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "rental_east" => Ok(Self::RentalEast),
            "business_east" => Ok(Self::BusinessEast),
            "mixed_east" => Ok(Self::MixedEast),
            "undeveloped_east" => Ok(Self::UndevelopedEast),
            "other" => Ok(Self::Other),
            "agricultural" => Ok(Self::Agricultural),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1DeclarationsDeReturnFactsSetRequestFactsLandHoldingsItemCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RentalEast => write!(f, "rental_east"),
            Self::BusinessEast => write!(f, "business_east"),
            Self::MixedEast => write!(f, "mixed_east"),
            Self::UndevelopedEast => write!(f, "undeveloped_east"),
            Self::Other => write!(f, "other"),
            Self::Agricultural => write!(f, "agricultural"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
