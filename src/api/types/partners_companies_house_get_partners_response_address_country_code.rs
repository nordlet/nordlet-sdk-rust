pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CompaniesHouseGetPartnersResponseAddressCountryCode {
    #[serde(rename = "GB")]
    Gb,
}
impl fmt::Display for CompaniesHouseGetPartnersResponseAddressCountryCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Gb => "GB",
        };
        write!(f, "{}", s)
    }
}
