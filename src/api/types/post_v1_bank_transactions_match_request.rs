pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1BankTransactionsMatchRequest {
    #[serde(rename = "transactionId")]
    #[serde(default)]
    pub transaction_id: String,
    #[serde(rename = "documentType")]
    pub document_type: PostV1BankTransactionsMatchRequestDocumentType,
    #[serde(rename = "documentId")]
    #[serde(default)]
    pub document_id: String,
}

impl PostV1BankTransactionsMatchRequest {
    pub fn builder() -> PostV1BankTransactionsMatchRequestBuilder {
        <PostV1BankTransactionsMatchRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankTransactionsMatchRequestBuilder {
    transaction_id: Option<String>,
    document_type: Option<PostV1BankTransactionsMatchRequestDocumentType>,
    document_id: Option<String>,
}

impl PostV1BankTransactionsMatchRequestBuilder {
    pub fn transaction_id(mut self, value: impl Into<String>) -> Self {
        self.transaction_id = Some(value.into());
        self
    }

    pub fn document_type(mut self, value: PostV1BankTransactionsMatchRequestDocumentType) -> Self {
        self.document_type = Some(value);
        self
    }

    pub fn document_id(mut self, value: impl Into<String>) -> Self {
        self.document_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankTransactionsMatchRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`transaction_id`](PostV1BankTransactionsMatchRequestBuilder::transaction_id)
    /// - [`document_type`](PostV1BankTransactionsMatchRequestBuilder::document_type)
    /// - [`document_id`](PostV1BankTransactionsMatchRequestBuilder::document_id)
    pub fn build(self) -> Result<PostV1BankTransactionsMatchRequest, BuildError> {
        Ok(PostV1BankTransactionsMatchRequest {
            transaction_id: self
                .transaction_id
                .ok_or_else(|| BuildError::missing_field("transaction_id"))?,
            document_type: self
                .document_type
                .ok_or_else(|| BuildError::missing_field("document_type"))?,
            document_id: self
                .document_id
                .ok_or_else(|| BuildError::missing_field("document_id"))?,
        })
    }
}
