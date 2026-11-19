pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RunsListPayrollResponseRowsItemFrequency {
    Monthly,
    Semimonthly,
    FourWeekly,
    Biweekly,
    Weekly,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RunsListPayrollResponseRowsItemFrequency {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Monthly => serializer.serialize_str("monthly"),
            Self::Semimonthly => serializer.serialize_str("semimonthly"),
            Self::FourWeekly => serializer.serialize_str("four_weekly"),
            Self::Biweekly => serializer.serialize_str("biweekly"),
            Self::Weekly => serializer.serialize_str("weekly"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for RunsListPayrollResponseRowsItemFrequency {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "monthly" => Ok(Self::Monthly),
            "semimonthly" => Ok(Self::Semimonthly),
            "four_weekly" => Ok(Self::FourWeekly),
            "biweekly" => Ok(Self::Biweekly),
            "weekly" => Ok(Self::Weekly),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RunsListPayrollResponseRowsItemFrequency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Monthly => write!(f, "monthly"),
            Self::Semimonthly => write!(f, "semimonthly"),
            Self::FourWeekly => write!(f, "four_weekly"),
            Self::Biweekly => write!(f, "biweekly"),
            Self::Weekly => write!(f, "weekly"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
