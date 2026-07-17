pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CatalogPriceListsListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1CatalogPriceListsListResponseRowsItem>,
}

impl PostV1CatalogPriceListsListResponse {
    pub fn builder() -> PostV1CatalogPriceListsListResponseBuilder {
        <PostV1CatalogPriceListsListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CatalogPriceListsListResponseBuilder {
    rows: Option<Vec<PostV1CatalogPriceListsListResponseRowsItem>>,
}

impl PostV1CatalogPriceListsListResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1CatalogPriceListsListResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1CatalogPriceListsListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1CatalogPriceListsListResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1CatalogPriceListsListResponse, BuildError> {
        Ok(PostV1CatalogPriceListsListResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
