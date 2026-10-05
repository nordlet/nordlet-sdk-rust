pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SettlementsLinkBankRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "bankTransactionId")]
    #[serde(default)]
    pub bank_transaction_id: String,
}

impl SettlementsLinkBankRequest {
    pub fn builder() -> SettlementsLinkBankRequestBuilder {
        <SettlementsLinkBankRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SettlementsLinkBankRequestBuilder {
    id: Option<String>,
    bank_transaction_id: Option<String>,
}

impl SettlementsLinkBankRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn bank_transaction_id(mut self, value: impl Into<String>) -> Self {
        self.bank_transaction_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SettlementsLinkBankRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SettlementsLinkBankRequestBuilder::id)
    /// - [`bank_transaction_id`](SettlementsLinkBankRequestBuilder::bank_transaction_id)
    pub fn build(self) -> Result<SettlementsLinkBankRequest, BuildError> {
        Ok(SettlementsLinkBankRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            bank_transaction_id: self
                .bank_transaction_id
                .ok_or_else(|| BuildError::missing_field("bank_transaction_id"))?,
        })
    }
}
