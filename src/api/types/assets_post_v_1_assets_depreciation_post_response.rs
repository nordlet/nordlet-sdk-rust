pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AssetsDepreciationPostResponse {
    #[serde(default)]
    pub posted: i64,
    #[serde(default)]
    pub skipped: i64,
    #[serde(default)]
    pub total: String,
    #[serde(rename = "journalTransactionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journal_transaction_id: Option<String>,
}

impl PostV1AssetsDepreciationPostResponse {
    pub fn builder() -> PostV1AssetsDepreciationPostResponseBuilder {
        <PostV1AssetsDepreciationPostResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AssetsDepreciationPostResponseBuilder {
    posted: Option<i64>,
    skipped: Option<i64>,
    total: Option<String>,
    journal_transaction_id: Option<String>,
}

impl PostV1AssetsDepreciationPostResponseBuilder {
    pub fn posted(mut self, value: i64) -> Self {
        self.posted = Some(value);
        self
    }

    pub fn skipped(mut self, value: i64) -> Self {
        self.skipped = Some(value);
        self
    }

    pub fn total(mut self, value: impl Into<String>) -> Self {
        self.total = Some(value.into());
        self
    }

    pub fn journal_transaction_id(mut self, value: impl Into<String>) -> Self {
        self.journal_transaction_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AssetsDepreciationPostResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`posted`](PostV1AssetsDepreciationPostResponseBuilder::posted)
    /// - [`skipped`](PostV1AssetsDepreciationPostResponseBuilder::skipped)
    /// - [`total`](PostV1AssetsDepreciationPostResponseBuilder::total)
    pub fn build(self) -> Result<PostV1AssetsDepreciationPostResponse, BuildError> {
        Ok(PostV1AssetsDepreciationPostResponse {
            posted: self
                .posted
                .ok_or_else(|| BuildError::missing_field("posted"))?,
            skipped: self
                .skipped
                .ok_or_else(|| BuildError::missing_field("skipped"))?,
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
            journal_transaction_id: self.journal_transaction_id,
        })
    }
}
