pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SettlementsImportBankRequest {
    #[serde(rename = "bankAccountId")]
    #[serde(default)]
    pub bank_account_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<SettlementsImportBankRequestProvider>,
    #[serde(default)]
    pub content: String,
}

impl SettlementsImportBankRequest {
    pub fn builder() -> SettlementsImportBankRequestBuilder {
        <SettlementsImportBankRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SettlementsImportBankRequestBuilder {
    bank_account_id: Option<String>,
    provider: Option<SettlementsImportBankRequestProvider>,
    content: Option<String>,
}

impl SettlementsImportBankRequestBuilder {
    pub fn bank_account_id(mut self, value: impl Into<String>) -> Self {
        self.bank_account_id = Some(value.into());
        self
    }

    pub fn provider(mut self, value: SettlementsImportBankRequestProvider) -> Self {
        self.provider = Some(value);
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SettlementsImportBankRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`bank_account_id`](SettlementsImportBankRequestBuilder::bank_account_id)
    /// - [`content`](SettlementsImportBankRequestBuilder::content)
    pub fn build(self) -> Result<SettlementsImportBankRequest, BuildError> {
        Ok(SettlementsImportBankRequest {
            bank_account_id: self
                .bank_account_id
                .ok_or_else(|| BuildError::missing_field("bank_account_id"))?,
            provider: self.provider,
            content: self
                .content
                .ok_or_else(|| BuildError::missing_field("content"))?,
        })
    }
}
