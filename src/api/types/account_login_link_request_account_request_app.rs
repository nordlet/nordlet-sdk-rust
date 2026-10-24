pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum LoginLinkRequestAccountRequestApp {
    #[serde(rename = "invoice")]
    Invoice,
}
impl fmt::Display for LoginLinkRequestAccountRequestApp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Invoice => "invoice",
        };
        write!(f, "{}", s)
    }
}
