pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1SalesRecognitionRunsListResponseRowsItemTrigger {
    Manual,
    ScheduleDue,
    PeriodClose,
    DeliveryAct,
    Progress,
    Modification,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1SalesRecognitionRunsListResponseRowsItemTrigger {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Manual => serializer.serialize_str("manual"),
            Self::ScheduleDue => serializer.serialize_str("schedule_due"),
            Self::PeriodClose => serializer.serialize_str("period_close"),
            Self::DeliveryAct => serializer.serialize_str("delivery_act"),
            Self::Progress => serializer.serialize_str("progress"),
            Self::Modification => serializer.serialize_str("modification"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1SalesRecognitionRunsListResponseRowsItemTrigger {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "manual" => Ok(Self::Manual),
            "schedule_due" => Ok(Self::ScheduleDue),
            "period_close" => Ok(Self::PeriodClose),
            "delivery_act" => Ok(Self::DeliveryAct),
            "progress" => Ok(Self::Progress),
            "modification" => Ok(Self::Modification),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1SalesRecognitionRunsListResponseRowsItemTrigger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Manual => write!(f, "manual"),
            Self::ScheduleDue => write!(f, "schedule_due"),
            Self::PeriodClose => write!(f, "period_close"),
            Self::DeliveryAct => write!(f, "delivery_act"),
            Self::Progress => write!(f, "progress"),
            Self::Modification => write!(f, "modification"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
