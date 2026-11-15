pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CompaniesContextAccountResponseFilingsItemMissingSettingsOperation {
    #[serde(rename = "declarations/configs/update")]
    DeclarationsConfigsUpdate,
}
impl fmt::Display for CompaniesContextAccountResponseFilingsItemMissingSettingsOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::DeclarationsConfigsUpdate => "declarations/configs/update",
        };
        write!(f, "{}", s)
    }
}
