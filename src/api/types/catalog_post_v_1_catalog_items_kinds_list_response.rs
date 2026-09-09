pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CatalogItemsKindsListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1CatalogItemsKindsListResponseRowsItem>,
}

impl PostV1CatalogItemsKindsListResponse {
    pub fn builder() -> PostV1CatalogItemsKindsListResponseBuilder {
        <PostV1CatalogItemsKindsListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CatalogItemsKindsListResponseBuilder {
    rows: Option<Vec<PostV1CatalogItemsKindsListResponseRowsItem>>,
}

impl PostV1CatalogItemsKindsListResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1CatalogItemsKindsListResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1CatalogItemsKindsListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1CatalogItemsKindsListResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1CatalogItemsKindsListResponse, BuildError> {
        Ok(PostV1CatalogItemsKindsListResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
