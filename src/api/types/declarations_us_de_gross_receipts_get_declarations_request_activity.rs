pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UsDeGrossReceiptsGetDeclarationsRequestActivity {
    Services,
    Contractor,
    Manufacturer,
    CleanEnergyDeviceManufacturer,
    AutomobileManufacturer,
    Wholesaler,
    FoodProcessor,
    CommercialFeedDealer,
    Retailer,
    RestaurantRetailer,
    FarmMachineryRetailer,
    GrocerySupermarketRetailer,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UsDeGrossReceiptsGetDeclarationsRequestActivity {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Services => serializer.serialize_str("services"),
            Self::Contractor => serializer.serialize_str("contractor"),
            Self::Manufacturer => serializer.serialize_str("manufacturer"),
            Self::CleanEnergyDeviceManufacturer => {
                serializer.serialize_str("clean-energy-device-manufacturer")
            }
            Self::AutomobileManufacturer => serializer.serialize_str("automobile-manufacturer"),
            Self::Wholesaler => serializer.serialize_str("wholesaler"),
            Self::FoodProcessor => serializer.serialize_str("food-processor"),
            Self::CommercialFeedDealer => serializer.serialize_str("commercial-feed-dealer"),
            Self::Retailer => serializer.serialize_str("retailer"),
            Self::RestaurantRetailer => serializer.serialize_str("restaurant-retailer"),
            Self::FarmMachineryRetailer => serializer.serialize_str("farm-machinery-retailer"),
            Self::GrocerySupermarketRetailer => {
                serializer.serialize_str("grocery-supermarket-retailer")
            }
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UsDeGrossReceiptsGetDeclarationsRequestActivity {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "services" => Ok(Self::Services),
            "contractor" => Ok(Self::Contractor),
            "manufacturer" => Ok(Self::Manufacturer),
            "clean-energy-device-manufacturer" => Ok(Self::CleanEnergyDeviceManufacturer),
            "automobile-manufacturer" => Ok(Self::AutomobileManufacturer),
            "wholesaler" => Ok(Self::Wholesaler),
            "food-processor" => Ok(Self::FoodProcessor),
            "commercial-feed-dealer" => Ok(Self::CommercialFeedDealer),
            "retailer" => Ok(Self::Retailer),
            "restaurant-retailer" => Ok(Self::RestaurantRetailer),
            "farm-machinery-retailer" => Ok(Self::FarmMachineryRetailer),
            "grocery-supermarket-retailer" => Ok(Self::GrocerySupermarketRetailer),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UsDeGrossReceiptsGetDeclarationsRequestActivity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Services => write!(f, "services"),
            Self::Contractor => write!(f, "contractor"),
            Self::Manufacturer => write!(f, "manufacturer"),
            Self::CleanEnergyDeviceManufacturer => write!(f, "clean-energy-device-manufacturer"),
            Self::AutomobileManufacturer => write!(f, "automobile-manufacturer"),
            Self::Wholesaler => write!(f, "wholesaler"),
            Self::FoodProcessor => write!(f, "food-processor"),
            Self::CommercialFeedDealer => write!(f, "commercial-feed-dealer"),
            Self::Retailer => write!(f, "retailer"),
            Self::RestaurantRetailer => write!(f, "restaurant-retailer"),
            Self::FarmMachineryRetailer => write!(f, "farm-machinery-retailer"),
            Self::GrocerySupermarketRetailer => write!(f, "grocery-supermarket-retailer"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
