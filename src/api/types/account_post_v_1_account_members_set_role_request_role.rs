pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1AccountMembersSetRoleRequestRole {
    Admin,
    Accountant,
    Manager,
    Developer,
    Viewer,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1AccountMembersSetRoleRequestRole {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Admin => serializer.serialize_str("admin"),
            Self::Accountant => serializer.serialize_str("accountant"),
            Self::Manager => serializer.serialize_str("manager"),
            Self::Developer => serializer.serialize_str("developer"),
            Self::Viewer => serializer.serialize_str("viewer"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1AccountMembersSetRoleRequestRole {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "admin" => Ok(Self::Admin),
            "accountant" => Ok(Self::Accountant),
            "manager" => Ok(Self::Manager),
            "developer" => Ok(Self::Developer),
            "viewer" => Ok(Self::Viewer),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1AccountMembersSetRoleRequestRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Admin => write!(f, "admin"),
            Self::Accountant => write!(f, "accountant"),
            Self::Manager => write!(f, "manager"),
            Self::Developer => write!(f, "developer"),
            Self::Viewer => write!(f, "viewer"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
