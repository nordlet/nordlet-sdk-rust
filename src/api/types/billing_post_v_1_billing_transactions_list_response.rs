pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BillingTransactionsListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1BillingTransactionsListResponseRowsItem>,
}

impl PostV1BillingTransactionsListResponse {
    pub fn builder() -> PostV1BillingTransactionsListResponseBuilder {
        <PostV1BillingTransactionsListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BillingTransactionsListResponseBuilder {
    rows: Option<Vec<PostV1BillingTransactionsListResponseRowsItem>>,
}

impl PostV1BillingTransactionsListResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1BillingTransactionsListResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1BillingTransactionsListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1BillingTransactionsListResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1BillingTransactionsListResponse, BuildError> {
        Ok(PostV1BillingTransactionsListResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
