pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CatalogUnitsListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1CatalogUnitsListResponseRowsItem>,
}

impl PostV1CatalogUnitsListResponse {
    pub fn builder() -> PostV1CatalogUnitsListResponseBuilder {
        <PostV1CatalogUnitsListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CatalogUnitsListResponseBuilder {
    rows: Option<Vec<PostV1CatalogUnitsListResponseRowsItem>>,
}

impl PostV1CatalogUnitsListResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1CatalogUnitsListResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1CatalogUnitsListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1CatalogUnitsListResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1CatalogUnitsListResponse, BuildError> {
        Ok(PostV1CatalogUnitsListResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
