pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceBanksListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1ReferenceBanksListResponseRowsItem>,
    #[serde(default)]
    pub page: i64,
    #[serde(rename = "pageSize")]
    #[serde(default)]
    pub page_size: i64,
    #[serde(default)]
    pub total: i64,
}

impl PostV1ReferenceBanksListResponse {
    pub fn builder() -> PostV1ReferenceBanksListResponseBuilder {
        <PostV1ReferenceBanksListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceBanksListResponseBuilder {
    rows: Option<Vec<PostV1ReferenceBanksListResponseRowsItem>>,
    page: Option<i64>,
    page_size: Option<i64>,
    total: Option<i64>,
}

impl PostV1ReferenceBanksListResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1ReferenceBanksListResponseRowsItem>) -> Self {
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

    /// Consumes the builder and constructs a [`PostV1ReferenceBanksListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1ReferenceBanksListResponseBuilder::rows)
    /// - [`page`](PostV1ReferenceBanksListResponseBuilder::page)
    /// - [`page_size`](PostV1ReferenceBanksListResponseBuilder::page_size)
    /// - [`total`](PostV1ReferenceBanksListResponseBuilder::total)
    pub fn build(self) -> Result<PostV1ReferenceBanksListResponse, BuildError> {
        Ok(PostV1ReferenceBanksListResponse {
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
