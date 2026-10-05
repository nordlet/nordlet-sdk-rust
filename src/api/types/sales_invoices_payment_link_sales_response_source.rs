pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum InvoicesPaymentLinkSalesResponseSource {
    #[serde(rename = "template")]
    Template,
}
impl fmt::Display for InvoicesPaymentLinkSalesResponseSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Template => "template",
        };
        write!(f, "{}", s)
    }
}
