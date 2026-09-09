pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostV1SalesInvoicesPaymentLinkResponseSource {
    #[serde(rename = "template")]
    Template,
}
impl fmt::Display for PostV1SalesInvoicesPaymentLinkResponseSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Template => "template",
        };
        write!(f, "{}", s)
    }
}
