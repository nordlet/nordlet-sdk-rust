pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ItemsKindsDeleteCatalogResponse {
    #[serde(default)]
    pub id: String,
}

impl ItemsKindsDeleteCatalogResponse {
    pub fn builder() -> ItemsKindsDeleteCatalogResponseBuilder {
        <ItemsKindsDeleteCatalogResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ItemsKindsDeleteCatalogResponseBuilder {
    id: Option<String>,
}

impl ItemsKindsDeleteCatalogResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ItemsKindsDeleteCatalogResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ItemsKindsDeleteCatalogResponseBuilder::id)
    pub fn build(self) -> Result<ItemsKindsDeleteCatalogResponse, BuildError> {
        Ok(ItemsKindsDeleteCatalogResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
