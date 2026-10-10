pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WebhooksPeppolRequestProvider {
    Recommand,
    Storecove,
    EInvoiceBe,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for WebhooksPeppolRequestProvider {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Recommand => serializer.serialize_str("recommand"),
            Self::Storecove => serializer.serialize_str("storecove"),
            Self::EInvoiceBe => serializer.serialize_str("e-invoice-be"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for WebhooksPeppolRequestProvider {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "recommand" => Ok(Self::Recommand),
            "storecove" => Ok(Self::Storecove),
            "e-invoice-be" => Ok(Self::EInvoiceBe),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for WebhooksPeppolRequestProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Recommand => write!(f, "recommand"),
            Self::Storecove => write!(f, "storecove"),
            Self::EInvoiceBe => write!(f, "e-invoice-be"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
