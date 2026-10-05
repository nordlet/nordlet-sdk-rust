pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SubmissionsCreateDeclarationsRequestObligation {
    #[serde(rename = "lt-isaf")]
    LtIsaf,
}
impl fmt::Display for SubmissionsCreateDeclarationsRequestObligation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::LtIsaf => "lt-isaf",
        };
        write!(f, "{}", s)
    }
}
