pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CatalogItemsSuppliersListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1CatalogItemsSuppliersListResponseRowsItem>,
}

impl PostV1CatalogItemsSuppliersListResponse {
    pub fn builder() -> PostV1CatalogItemsSuppliersListResponseBuilder {
        <PostV1CatalogItemsSuppliersListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CatalogItemsSuppliersListResponseBuilder {
    rows: Option<Vec<PostV1CatalogItemsSuppliersListResponseRowsItem>>,
}

impl PostV1CatalogItemsSuppliersListResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1CatalogItemsSuppliersListResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1CatalogItemsSuppliersListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1CatalogItemsSuppliersListResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1CatalogItemsSuppliersListResponse, BuildError> {
        Ok(PostV1CatalogItemsSuppliersListResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
