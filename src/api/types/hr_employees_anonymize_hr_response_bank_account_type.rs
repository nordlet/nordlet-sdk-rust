pub use crate::prelude::*;

/// Checking or savings account; a US ACH file needs it
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EmployeesAnonymizeHrResponseBankAccountType {
    Checking,
    Savings,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for EmployeesAnonymizeHrResponseBankAccountType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Checking => serializer.serialize_str("checking"),
            Self::Savings => serializer.serialize_str("savings"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for EmployeesAnonymizeHrResponseBankAccountType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "checking" => Ok(Self::Checking),
            "savings" => Ok(Self::Savings),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for EmployeesAnonymizeHrResponseBankAccountType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Checking => write!(f, "checking"),
            Self::Savings => write!(f, "savings"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
