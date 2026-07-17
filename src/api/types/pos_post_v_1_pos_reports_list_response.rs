pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PosReportsListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1PosReportsListResponseRowsItem>,
    #[serde(default)]
    pub page: i64,
    #[serde(rename = "pageSize")]
    #[serde(default)]
    pub page_size: i64,
    #[serde(default)]
    pub total: i64,
}

impl PostV1PosReportsListResponse {
    pub fn builder() -> PostV1PosReportsListResponseBuilder {
        <PostV1PosReportsListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PosReportsListResponseBuilder {
    rows: Option<Vec<PostV1PosReportsListResponseRowsItem>>,
    page: Option<i64>,
    page_size: Option<i64>,
    total: Option<i64>,
}

impl PostV1PosReportsListResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1PosReportsListResponseRowsItem>) -> Self {
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

    /// Consumes the builder and constructs a [`PostV1PosReportsListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1PosReportsListResponseBuilder::rows)
    /// - [`page`](PostV1PosReportsListResponseBuilder::page)
    /// - [`page_size`](PostV1PosReportsListResponseBuilder::page_size)
    /// - [`total`](PostV1PosReportsListResponseBuilder::total)
    pub fn build(self) -> Result<PostV1PosReportsListResponse, BuildError> {
        Ok(PostV1PosReportsListResponse {
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
