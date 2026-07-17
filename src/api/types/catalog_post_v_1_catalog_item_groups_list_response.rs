pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CatalogItemGroupsListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1CatalogItemGroupsListResponseRowsItem>,
}

impl PostV1CatalogItemGroupsListResponse {
    pub fn builder() -> PostV1CatalogItemGroupsListResponseBuilder {
        <PostV1CatalogItemGroupsListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CatalogItemGroupsListResponseBuilder {
    rows: Option<Vec<PostV1CatalogItemGroupsListResponseRowsItem>>,
}

impl PostV1CatalogItemGroupsListResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1CatalogItemGroupsListResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1CatalogItemGroupsListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1CatalogItemGroupsListResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1CatalogItemGroupsListResponse, BuildError> {
        Ok(PostV1CatalogItemGroupsListResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
