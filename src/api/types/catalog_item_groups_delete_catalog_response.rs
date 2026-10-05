pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ItemGroupsDeleteCatalogResponse {
    #[serde(default)]
    pub id: String,
}

impl ItemGroupsDeleteCatalogResponse {
    pub fn builder() -> ItemGroupsDeleteCatalogResponseBuilder {
        <ItemGroupsDeleteCatalogResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ItemGroupsDeleteCatalogResponseBuilder {
    id: Option<String>,
}

impl ItemGroupsDeleteCatalogResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ItemGroupsDeleteCatalogResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ItemGroupsDeleteCatalogResponseBuilder::id)
    pub fn build(self) -> Result<ItemGroupsDeleteCatalogResponse, BuildError> {
        Ok(ItemGroupsDeleteCatalogResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
