pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CatalogPriceListsItemsListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1CatalogPriceListsItemsListResponseRowsItem>,
}

impl PostV1CatalogPriceListsItemsListResponse {
    pub fn builder() -> PostV1CatalogPriceListsItemsListResponseBuilder {
        <PostV1CatalogPriceListsItemsListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CatalogPriceListsItemsListResponseBuilder {
    rows: Option<Vec<PostV1CatalogPriceListsItemsListResponseRowsItem>>,
}

impl PostV1CatalogPriceListsItemsListResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1CatalogPriceListsItemsListResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1CatalogPriceListsItemsListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1CatalogPriceListsItemsListResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1CatalogPriceListsItemsListResponse, BuildError> {
        Ok(PostV1CatalogPriceListsItemsListResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
