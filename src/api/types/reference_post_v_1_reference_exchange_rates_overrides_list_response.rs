pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceExchangeRatesOverridesListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1ReferenceExchangeRatesOverridesListResponseRowsItem>,
    #[serde(default)]
    pub page: i64,
    #[serde(rename = "pageSize")]
    #[serde(default)]
    pub page_size: i64,
    #[serde(default)]
    pub total: i64,
}

impl PostV1ReferenceExchangeRatesOverridesListResponse {
    pub fn builder() -> PostV1ReferenceExchangeRatesOverridesListResponseBuilder {
        <PostV1ReferenceExchangeRatesOverridesListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceExchangeRatesOverridesListResponseBuilder {
    rows: Option<Vec<PostV1ReferenceExchangeRatesOverridesListResponseRowsItem>>,
    page: Option<i64>,
    page_size: Option<i64>,
    total: Option<i64>,
}

impl PostV1ReferenceExchangeRatesOverridesListResponseBuilder {
    pub fn rows(
        mut self,
        value: Vec<PostV1ReferenceExchangeRatesOverridesListResponseRowsItem>,
    ) -> Self {
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

    /// Consumes the builder and constructs a [`PostV1ReferenceExchangeRatesOverridesListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1ReferenceExchangeRatesOverridesListResponseBuilder::rows)
    /// - [`page`](PostV1ReferenceExchangeRatesOverridesListResponseBuilder::page)
    /// - [`page_size`](PostV1ReferenceExchangeRatesOverridesListResponseBuilder::page_size)
    /// - [`total`](PostV1ReferenceExchangeRatesOverridesListResponseBuilder::total)
    pub fn build(self) -> Result<PostV1ReferenceExchangeRatesOverridesListResponse, BuildError> {
        Ok(PostV1ReferenceExchangeRatesOverridesListResponse {
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
