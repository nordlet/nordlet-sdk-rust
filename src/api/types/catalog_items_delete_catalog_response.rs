pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ItemsDeleteCatalogResponse {
    #[serde(default)]
    pub id: String,
}

impl ItemsDeleteCatalogResponse {
    pub fn builder() -> ItemsDeleteCatalogResponseBuilder {
        <ItemsDeleteCatalogResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ItemsDeleteCatalogResponseBuilder {
    id: Option<String>,
}

impl ItemsDeleteCatalogResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ItemsDeleteCatalogResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ItemsDeleteCatalogResponseBuilder::id)
    pub fn build(self) -> Result<ItemsDeleteCatalogResponse, BuildError> {
        Ok(ItemsDeleteCatalogResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
