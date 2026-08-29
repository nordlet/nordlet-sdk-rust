pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostV1BillingUsageListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1BillingUsageListResponseRowsItem>,
}

impl PostV1BillingUsageListResponse {
    pub fn builder() -> PostV1BillingUsageListResponseBuilder {
        <PostV1BillingUsageListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BillingUsageListResponseBuilder {
    rows: Option<Vec<PostV1BillingUsageListResponseRowsItem>>,
}

impl PostV1BillingUsageListResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1BillingUsageListResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1BillingUsageListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1BillingUsageListResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1BillingUsageListResponse, BuildError> {
        Ok(PostV1BillingUsageListResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
