pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BankSettlementsImportRequest {
    #[serde(rename = "bankAccountId")]
    #[serde(default)]
    pub bank_account_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<PostV1BankSettlementsImportRequestProvider>,
    #[serde(default)]
    pub content: String,
}

impl PostV1BankSettlementsImportRequest {
    pub fn builder() -> PostV1BankSettlementsImportRequestBuilder {
        <PostV1BankSettlementsImportRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankSettlementsImportRequestBuilder {
    bank_account_id: Option<String>,
    provider: Option<PostV1BankSettlementsImportRequestProvider>,
    content: Option<String>,
}

impl PostV1BankSettlementsImportRequestBuilder {
    pub fn bank_account_id(mut self, value: impl Into<String>) -> Self {
        self.bank_account_id = Some(value.into());
        self
    }

    pub fn provider(mut self, value: PostV1BankSettlementsImportRequestProvider) -> Self {
        self.provider = Some(value);
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankSettlementsImportRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`bank_account_id`](PostV1BankSettlementsImportRequestBuilder::bank_account_id)
    /// - [`content`](PostV1BankSettlementsImportRequestBuilder::content)
    pub fn build(self) -> Result<PostV1BankSettlementsImportRequest, BuildError> {
        Ok(PostV1BankSettlementsImportRequest {
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
