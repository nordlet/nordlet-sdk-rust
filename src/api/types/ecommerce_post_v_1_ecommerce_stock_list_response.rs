pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1EcommerceStockListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1EcommerceStockListResponseRowsItem>,
}

impl PostV1EcommerceStockListResponse {
    pub fn builder() -> PostV1EcommerceStockListResponseBuilder {
        <PostV1EcommerceStockListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1EcommerceStockListResponseBuilder {
    rows: Option<Vec<PostV1EcommerceStockListResponseRowsItem>>,
}

impl PostV1EcommerceStockListResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1EcommerceStockListResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1EcommerceStockListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1EcommerceStockListResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1EcommerceStockListResponse, BuildError> {
        Ok(PostV1EcommerceStockListResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
