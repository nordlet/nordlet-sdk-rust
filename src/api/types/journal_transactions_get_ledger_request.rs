pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct JournalTransactionsGetLedgerRequest {
    #[serde(default)]
    pub id: String,
}

impl JournalTransactionsGetLedgerRequest {
    pub fn builder() -> JournalTransactionsGetLedgerRequestBuilder {
        <JournalTransactionsGetLedgerRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct JournalTransactionsGetLedgerRequestBuilder {
    id: Option<String>,
}

impl JournalTransactionsGetLedgerRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`JournalTransactionsGetLedgerRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](JournalTransactionsGetLedgerRequestBuilder::id)
    pub fn build(self) -> Result<JournalTransactionsGetLedgerRequest, BuildError> {
        Ok(JournalTransactionsGetLedgerRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
