pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ProductionOrdersListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1ProductionOrdersListResponseRowsItem>,
    #[serde(default)]
    pub page: i64,
    #[serde(rename = "pageSize")]
    #[serde(default)]
    pub page_size: i64,
    #[serde(default)]
    pub total: i64,
}

impl PostV1ProductionOrdersListResponse {
    pub fn builder() -> PostV1ProductionOrdersListResponseBuilder {
        <PostV1ProductionOrdersListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProductionOrdersListResponseBuilder {
    rows: Option<Vec<PostV1ProductionOrdersListResponseRowsItem>>,
    page: Option<i64>,
    page_size: Option<i64>,
    total: Option<i64>,
}

impl PostV1ProductionOrdersListResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1ProductionOrdersListResponseRowsItem>) -> Self {
        self.rows = Some(value);
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

    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ProductionOrdersListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1ProductionOrdersListResponseBuilder::rows)
    /// - [`page`](PostV1ProductionOrdersListResponseBuilder::page)
    /// - [`page_size`](PostV1ProductionOrdersListResponseBuilder::page_size)
    /// - [`total`](PostV1ProductionOrdersListResponseBuilder::total)
    pub fn build(self) -> Result<PostV1ProductionOrdersListResponse, BuildError> {
        Ok(PostV1ProductionOrdersListResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
            page: self.page.ok_or_else(|| BuildError::missing_field("page"))?,
            page_size: self
                .page_size
                .ok_or_else(|| BuildError::missing_field("page_size"))?,
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
        })
    }
}
