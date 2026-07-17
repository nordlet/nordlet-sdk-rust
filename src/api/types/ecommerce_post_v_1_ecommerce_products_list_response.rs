pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostV1EcommerceProductsListResponse {
    #[serde(default)]
    pub total: i64,
    #[serde(default)]
    pub page: i64,
    #[serde(rename = "pageSize")]
    #[serde(default)]
    pub page_size: i64,
    #[serde(default)]
    pub rows: Vec<PostV1EcommerceProductsListResponseRowsItem>,
}

impl PostV1EcommerceProductsListResponse {
    pub fn builder() -> PostV1EcommerceProductsListResponseBuilder {
        <PostV1EcommerceProductsListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1EcommerceProductsListResponseBuilder {
    total: Option<i64>,
    page: Option<i64>,
    page_size: Option<i64>,
    rows: Option<Vec<PostV1EcommerceProductsListResponseRowsItem>>,
}

impl PostV1EcommerceProductsListResponseBuilder {
    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn rows(mut self, value: Vec<PostV1EcommerceProductsListResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1EcommerceProductsListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`total`](PostV1EcommerceProductsListResponseBuilder::total)
    /// - [`page`](PostV1EcommerceProductsListResponseBuilder::page)
    /// - [`page_size`](PostV1EcommerceProductsListResponseBuilder::page_size)
    /// - [`rows`](PostV1EcommerceProductsListResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1EcommerceProductsListResponse, BuildError> {
        Ok(PostV1EcommerceProductsListResponse {
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
            page: self.page.ok_or_else(|| BuildError::missing_field("page"))?,
            page_size: self
                .page_size
                .ok_or_else(|| BuildError::missing_field("page_size"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
