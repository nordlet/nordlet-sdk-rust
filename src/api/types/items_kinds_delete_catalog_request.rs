pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ItemsKindsDeleteCatalogRequest {
    #[serde(default)]
    pub id: String,
}

impl ItemsKindsDeleteCatalogRequest {
    pub fn builder() -> ItemsKindsDeleteCatalogRequestBuilder {
        <ItemsKindsDeleteCatalogRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ItemsKindsDeleteCatalogRequestBuilder {
    id: Option<String>,
}

impl ItemsKindsDeleteCatalogRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ItemsKindsDeleteCatalogRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ItemsKindsDeleteCatalogRequestBuilder::id)
    pub fn build(self) -> Result<ItemsKindsDeleteCatalogRequest, BuildError> {
        Ok(ItemsKindsDeleteCatalogRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
