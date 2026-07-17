pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BankStatementsImportRequest {
    #[serde(rename = "bankAccountId")]
    #[serde(default)]
    pub bank_account_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<PostV1BankStatementsImportRequestFormat>,
    #[serde(default)]
    pub content: String,
}

impl PostV1BankStatementsImportRequest {
    pub fn builder() -> PostV1BankStatementsImportRequestBuilder {
        <PostV1BankStatementsImportRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankStatementsImportRequestBuilder {
    bank_account_id: Option<String>,
    format: Option<PostV1BankStatementsImportRequestFormat>,
    content: Option<String>,
}

impl PostV1BankStatementsImportRequestBuilder {
    pub fn bank_account_id(mut self, value: impl Into<String>) -> Self {
        self.bank_account_id = Some(value.into());
        self
    }

    pub fn format(mut self, value: PostV1BankStatementsImportRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankStatementsImportRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`bank_account_id`](PostV1BankStatementsImportRequestBuilder::bank_account_id)
    /// - [`content`](PostV1BankStatementsImportRequestBuilder::content)
    pub fn build(self) -> Result<PostV1BankStatementsImportRequest, BuildError> {
        Ok(PostV1BankStatementsImportRequest {
            bank_account_id: self
                .bank_account_id
                .ok_or_else(|| BuildError::missing_field("bank_account_id"))?,
            format: self.format,
            content: self
                .content
                .ok_or_else(|| BuildError::missing_field("content"))?,
        })
    }
}
