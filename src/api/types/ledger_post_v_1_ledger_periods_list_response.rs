pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerPeriodsListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1LedgerPeriodsListResponseRowsItem>,
    #[serde(default)]
    pub page: i64,
    #[serde(rename = "pageSize")]
    #[serde(default)]
    pub page_size: i64,
    #[serde(default)]
    pub total: i64,
}

impl PostV1LedgerPeriodsListResponse {
    pub fn builder() -> PostV1LedgerPeriodsListResponseBuilder {
        <PostV1LedgerPeriodsListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerPeriodsListResponseBuilder {
    rows: Option<Vec<PostV1LedgerPeriodsListResponseRowsItem>>,
    page: Option<i64>,
    page_size: Option<i64>,
    total: Option<i64>,
}

impl PostV1LedgerPeriodsListResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1LedgerPeriodsListResponseRowsItem>) -> Self {
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

    /// Consumes the builder and constructs a [`PostV1LedgerPeriodsListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1LedgerPeriodsListResponseBuilder::rows)
    /// - [`page`](PostV1LedgerPeriodsListResponseBuilder::page)
    /// - [`page_size`](PostV1LedgerPeriodsListResponseBuilder::page_size)
    /// - [`total`](PostV1LedgerPeriodsListResponseBuilder::total)
    pub fn build(self) -> Result<PostV1LedgerPeriodsListResponse, BuildError> {
        Ok(PostV1LedgerPeriodsListResponse {
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
