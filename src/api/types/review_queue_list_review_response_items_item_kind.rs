pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QueueListReviewResponseItemsItemKind {
    DraftWarning,
    BankTransactionUnbooked,
    PartnerVatInvalid,
    FilingSettingsMissing,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for QueueListReviewResponseItemsItemKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::DraftWarning => serializer.serialize_str("draft_warning"),
            Self::BankTransactionUnbooked => serializer.serialize_str("bank_transaction_unbooked"),
            Self::PartnerVatInvalid => serializer.serialize_str("partner_vat_invalid"),
            Self::FilingSettingsMissing => serializer.serialize_str("filing_settings_missing"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for QueueListReviewResponseItemsItemKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "draft_warning" => Ok(Self::DraftWarning),
            "bank_transaction_unbooked" => Ok(Self::BankTransactionUnbooked),
            "partner_vat_invalid" => Ok(Self::PartnerVatInvalid),
            "filing_settings_missing" => Ok(Self::FilingSettingsMissing),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for QueueListReviewResponseItemsItemKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DraftWarning => write!(f, "draft_warning"),
            Self::BankTransactionUnbooked => write!(f, "bank_transaction_unbooked"),
            Self::PartnerVatInvalid => write!(f, "partner_vat_invalid"),
            Self::FilingSettingsMissing => write!(f, "filing_settings_missing"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
