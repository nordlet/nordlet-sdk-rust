pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BankSettlementsListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1BankSettlementsListResponseRowsItem>,
    #[serde(default)]
    pub page: i64,
    #[serde(rename = "pageSize")]
    #[serde(default)]
    pub page_size: i64,
    #[serde(default)]
    pub total: i64,
}

impl PostV1BankSettlementsListResponse {
    pub fn builder() -> PostV1BankSettlementsListResponseBuilder {
        <PostV1BankSettlementsListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankSettlementsListResponseBuilder {
    rows: Option<Vec<PostV1BankSettlementsListResponseRowsItem>>,
    page: Option<i64>,
    page_size: Option<i64>,
    total: Option<i64>,
}

impl PostV1BankSettlementsListResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1BankSettlementsListResponseRowsItem>) -> Self {
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

    /// Consumes the builder and constructs a [`PostV1BankSettlementsListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1BankSettlementsListResponseBuilder::rows)
    /// - [`page`](PostV1BankSettlementsListResponseBuilder::page)
    /// - [`page_size`](PostV1BankSettlementsListResponseBuilder::page_size)
    /// - [`total`](PostV1BankSettlementsListResponseBuilder::total)
    pub fn build(self) -> Result<PostV1BankSettlementsListResponse, BuildError> {
        Ok(PostV1BankSettlementsListResponse {
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
