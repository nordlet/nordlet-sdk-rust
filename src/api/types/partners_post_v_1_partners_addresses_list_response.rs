pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersAddressesListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1PartnersAddressesListResponseRowsItem>,
    #[serde(default)]
    pub page: i64,
    #[serde(rename = "pageSize")]
    #[serde(default)]
    pub page_size: i64,
    #[serde(default)]
    pub total: i64,
}

impl PostV1PartnersAddressesListResponse {
    pub fn builder() -> PostV1PartnersAddressesListResponseBuilder {
        <PostV1PartnersAddressesListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersAddressesListResponseBuilder {
    rows: Option<Vec<PostV1PartnersAddressesListResponseRowsItem>>,
    page: Option<i64>,
    page_size: Option<i64>,
    total: Option<i64>,
}

impl PostV1PartnersAddressesListResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1PartnersAddressesListResponseRowsItem>) -> Self {
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

    /// Consumes the builder and constructs a [`PostV1PartnersAddressesListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1PartnersAddressesListResponseBuilder::rows)
    /// - [`page`](PostV1PartnersAddressesListResponseBuilder::page)
    /// - [`page_size`](PostV1PartnersAddressesListResponseBuilder::page_size)
    /// - [`total`](PostV1PartnersAddressesListResponseBuilder::total)
    pub fn build(self) -> Result<PostV1PartnersAddressesListResponse, BuildError> {
        Ok(PostV1PartnersAddressesListResponse {
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
