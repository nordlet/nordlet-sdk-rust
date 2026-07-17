pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1HrTimesheetsListResponseRowsItemDaysItemType {
    Work,
    BusinessTrip,
    Vacation,
    Sick,
    Holiday,
    Unpaid,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1HrTimesheetsListResponseRowsItemDaysItemType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Work => serializer.serialize_str("work"),
            Self::BusinessTrip => serializer.serialize_str("business_trip"),
            Self::Vacation => serializer.serialize_str("vacation"),
            Self::Sick => serializer.serialize_str("sick"),
            Self::Holiday => serializer.serialize_str("holiday"),
            Self::Unpaid => serializer.serialize_str("unpaid"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1HrTimesheetsListResponseRowsItemDaysItemType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "work" => Ok(Self::Work),
            "business_trip" => Ok(Self::BusinessTrip),
            "vacation" => Ok(Self::Vacation),
            "sick" => Ok(Self::Sick),
            "holiday" => Ok(Self::Holiday),
            "unpaid" => Ok(Self::Unpaid),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1HrTimesheetsListResponseRowsItemDaysItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Work => write!(f, "work"),
            Self::BusinessTrip => write!(f, "business_trip"),
            Self::Vacation => write!(f, "vacation"),
            Self::Sick => write!(f, "sick"),
            Self::Holiday => write!(f, "holiday"),
            Self::Unpaid => write!(f, "unpaid"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
