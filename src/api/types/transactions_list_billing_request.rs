pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TransactionsListBillingRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl TransactionsListBillingRequest {
    pub fn builder() -> TransactionsListBillingRequestBuilder {
        <TransactionsListBillingRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransactionsListBillingRequestBuilder {
    limit: Option<i64>,
}

impl TransactionsListBillingRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TransactionsListBillingRequest`].
    pub fn build(self) -> Result<TransactionsListBillingRequest, BuildError> {
        Ok(TransactionsListBillingRequest { limit: self.limit })
    }
}
