pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostV1BankSettlementsImportRequestProvider {
    #[serde(rename = "stripe")]
    Stripe,
}
impl fmt::Display for PostV1BankSettlementsImportRequestProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Stripe => "stripe",
        };
        write!(f, "{}", s)
    }
}
