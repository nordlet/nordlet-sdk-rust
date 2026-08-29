pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ProjectsTimeEntriesListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1ProjectsTimeEntriesListResponseRowsItem>,
    #[serde(default)]
    pub page: i64,
    #[serde(rename = "pageSize")]
    #[serde(default)]
    pub page_size: i64,
    #[serde(default)]
    pub total: i64,
}

impl PostV1ProjectsTimeEntriesListResponse {
    pub fn builder() -> PostV1ProjectsTimeEntriesListResponseBuilder {
        <PostV1ProjectsTimeEntriesListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProjectsTimeEntriesListResponseBuilder {
    rows: Option<Vec<PostV1ProjectsTimeEntriesListResponseRowsItem>>,
    page: Option<i64>,
    page_size: Option<i64>,
    total: Option<i64>,
}

impl PostV1ProjectsTimeEntriesListResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1ProjectsTimeEntriesListResponseRowsItem>) -> Self {
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

    /// Consumes the builder and constructs a [`PostV1ProjectsTimeEntriesListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1ProjectsTimeEntriesListResponseBuilder::rows)
    /// - [`page`](PostV1ProjectsTimeEntriesListResponseBuilder::page)
    /// - [`page_size`](PostV1ProjectsTimeEntriesListResponseBuilder::page_size)
    /// - [`total`](PostV1ProjectsTimeEntriesListResponseBuilder::total)
    pub fn build(self) -> Result<PostV1ProjectsTimeEntriesListResponse, BuildError> {
        Ok(PostV1ProjectsTimeEntriesListResponse {
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
