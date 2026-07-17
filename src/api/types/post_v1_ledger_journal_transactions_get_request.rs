pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerJournalTransactionsGetRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1LedgerJournalTransactionsGetRequest {
    pub fn builder() -> PostV1LedgerJournalTransactionsGetRequestBuilder {
        <PostV1LedgerJournalTransactionsGetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerJournalTransactionsGetRequestBuilder {
    id: Option<String>,
}

impl PostV1LedgerJournalTransactionsGetRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerJournalTransactionsGetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1LedgerJournalTransactionsGetRequestBuilder::id)
    pub fn build(self) -> Result<PostV1LedgerJournalTransactionsGetRequest, BuildError> {
        Ok(PostV1LedgerJournalTransactionsGetRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
