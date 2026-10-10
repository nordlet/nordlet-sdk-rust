pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreatePlatformSellersRequestActivitiesItemActivity {
    ImmovableProperty,
    PersonalServices,
    SaleOfGoods,
    TransportationRental,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreatePlatformSellersRequestActivitiesItemActivity {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ImmovableProperty => serializer.serialize_str("immovable_property"),
            Self::PersonalServices => serializer.serialize_str("personal_services"),
            Self::SaleOfGoods => serializer.serialize_str("sale_of_goods"),
            Self::TransportationRental => serializer.serialize_str("transportation_rental"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreatePlatformSellersRequestActivitiesItemActivity {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "immovable_property" => Ok(Self::ImmovableProperty),
            "personal_services" => Ok(Self::PersonalServices),
            "sale_of_goods" => Ok(Self::SaleOfGoods),
            "transportation_rental" => Ok(Self::TransportationRental),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreatePlatformSellersRequestActivitiesItemActivity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ImmovableProperty => write!(f, "immovable_property"),
            Self::PersonalServices => write!(f, "personal_services"),
            Self::SaleOfGoods => write!(f, "sale_of_goods"),
            Self::TransportationRental => write!(f, "transportation_rental"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
