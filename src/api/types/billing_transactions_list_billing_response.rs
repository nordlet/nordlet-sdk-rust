pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TransactionsListBillingResponse {
    #[serde(default)]
    pub rows: Vec<TransactionsListBillingResponseRowsItem>,
}

impl TransactionsListBillingResponse {
    pub fn builder() -> TransactionsListBillingResponseBuilder {
        <TransactionsListBillingResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransactionsListBillingResponseBuilder {
    rows: Option<Vec<TransactionsListBillingResponseRowsItem>>,
}

impl TransactionsListBillingResponseBuilder {
    pub fn rows(mut self, value: Vec<TransactionsListBillingResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TransactionsListBillingResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](TransactionsListBillingResponseBuilder::rows)
    pub fn build(self) -> Result<TransactionsListBillingResponse, BuildError> {
        Ok(TransactionsListBillingResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
