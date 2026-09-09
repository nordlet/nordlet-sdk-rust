pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CatalogUnitsOptionsResponse {
    #[serde(default)]
    pub rows: Vec<PostV1CatalogUnitsOptionsResponseRowsItem>,
}

impl PostV1CatalogUnitsOptionsResponse {
    pub fn builder() -> PostV1CatalogUnitsOptionsResponseBuilder {
        <PostV1CatalogUnitsOptionsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CatalogUnitsOptionsResponseBuilder {
    rows: Option<Vec<PostV1CatalogUnitsOptionsResponseRowsItem>>,
}

impl PostV1CatalogUnitsOptionsResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1CatalogUnitsOptionsResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1CatalogUnitsOptionsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1CatalogUnitsOptionsResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1CatalogUnitsOptionsResponse, BuildError> {
        Ok(PostV1CatalogUnitsOptionsResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
