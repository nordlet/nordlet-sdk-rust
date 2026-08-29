pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BillingTransactionsListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl PostV1BillingTransactionsListRequest {
    pub fn builder() -> PostV1BillingTransactionsListRequestBuilder {
        <PostV1BillingTransactionsListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BillingTransactionsListRequestBuilder {
    limit: Option<i64>,
}

impl PostV1BillingTransactionsListRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1BillingTransactionsListRequest`].
    pub fn build(self) -> Result<PostV1BillingTransactionsListRequest, BuildError> {
        Ok(PostV1BillingTransactionsListRequest { limit: self.limit })
    }
}
