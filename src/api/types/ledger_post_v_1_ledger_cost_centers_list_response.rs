pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerCostCentersListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1LedgerCostCentersListResponseRowsItem>,
    #[serde(default)]
    pub page: i64,
    #[serde(rename = "pageSize")]
    #[serde(default)]
    pub page_size: i64,
    #[serde(default)]
    pub total: i64,
}

impl PostV1LedgerCostCentersListResponse {
    pub fn builder() -> PostV1LedgerCostCentersListResponseBuilder {
        <PostV1LedgerCostCentersListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerCostCentersListResponseBuilder {
    rows: Option<Vec<PostV1LedgerCostCentersListResponseRowsItem>>,
    page: Option<i64>,
    page_size: Option<i64>,
    total: Option<i64>,
}

impl PostV1LedgerCostCentersListResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1LedgerCostCentersListResponseRowsItem>) -> Self {
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

    /// Consumes the builder and constructs a [`PostV1LedgerCostCentersListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1LedgerCostCentersListResponseBuilder::rows)
    /// - [`page`](PostV1LedgerCostCentersListResponseBuilder::page)
    /// - [`page_size`](PostV1LedgerCostCentersListResponseBuilder::page_size)
    /// - [`total`](PostV1LedgerCostCentersListResponseBuilder::total)
    pub fn build(self) -> Result<PostV1LedgerCostCentersListResponse, BuildError> {
        Ok(PostV1LedgerCostCentersListResponse {
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
