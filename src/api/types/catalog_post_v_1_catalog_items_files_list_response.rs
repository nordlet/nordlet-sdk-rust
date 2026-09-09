pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CatalogItemsFilesListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1CatalogItemsFilesListResponseRowsItem>,
}

impl PostV1CatalogItemsFilesListResponse {
    pub fn builder() -> PostV1CatalogItemsFilesListResponseBuilder {
        <PostV1CatalogItemsFilesListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CatalogItemsFilesListResponseBuilder {
    rows: Option<Vec<PostV1CatalogItemsFilesListResponseRowsItem>>,
}

impl PostV1CatalogItemsFilesListResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1CatalogItemsFilesListResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1CatalogItemsFilesListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1CatalogItemsFilesListResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1CatalogItemsFilesListResponse, BuildError> {
        Ok(PostV1CatalogItemsFilesListResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
