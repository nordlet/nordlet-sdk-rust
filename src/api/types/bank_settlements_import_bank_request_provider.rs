pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SettlementsImportBankRequestProvider {
    #[serde(rename = "stripe")]
    Stripe,
}
impl fmt::Display for SettlementsImportBankRequestProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Stripe => "stripe",
        };
        write!(f, "{}", s)
    }
}
