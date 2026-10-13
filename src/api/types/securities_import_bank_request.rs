pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SecuritiesImportBankRequest {
    #[serde(rename = "bankAccountId")]
    #[serde(default)]
    pub bank_account_id: String,
    #[serde(default)]
    pub content: String,
    #[serde(rename = "fileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
}

impl SecuritiesImportBankRequest {
    pub fn builder() -> SecuritiesImportBankRequestBuilder {
        <SecuritiesImportBankRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SecuritiesImportBankRequestBuilder {
    bank_account_id: Option<String>,
    content: Option<String>,
    file_name: Option<String>,
}

impl SecuritiesImportBankRequestBuilder {
    pub fn bank_account_id(mut self, value: impl Into<String>) -> Self {
        self.bank_account_id = Some(value.into());
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.file_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SecuritiesImportBankRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`bank_account_id`](SecuritiesImportBankRequestBuilder::bank_account_id)
    /// - [`content`](SecuritiesImportBankRequestBuilder::content)
    pub fn build(self) -> Result<SecuritiesImportBankRequest, BuildError> {
        Ok(SecuritiesImportBankRequest {
            bank_account_id: self
                .bank_account_id
                .ok_or_else(|| BuildError::missing_field("bank_account_id"))?,
            content: self
                .content
                .ok_or_else(|| BuildError::missing_field("content"))?,
            file_name: self.file_name,
        })
    }
}
