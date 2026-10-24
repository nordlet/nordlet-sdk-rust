pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DebtRemindersUpcomingPartnersResponseRowsItemKind {
    BeforeDue,
    Overdue,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DebtRemindersUpcomingPartnersResponseRowsItemKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::BeforeDue => serializer.serialize_str("before_due"),
            Self::Overdue => serializer.serialize_str("overdue"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DebtRemindersUpcomingPartnersResponseRowsItemKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "before_due" => Ok(Self::BeforeDue),
            "overdue" => Ok(Self::Overdue),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DebtRemindersUpcomingPartnersResponseRowsItemKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BeforeDue => write!(f, "before_due"),
            Self::Overdue => write!(f, "overdue"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
