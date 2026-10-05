pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ItemsDeleteCatalogRequest {
    #[serde(default)]
    pub id: String,
}

impl ItemsDeleteCatalogRequest {
    pub fn builder() -> ItemsDeleteCatalogRequestBuilder {
        <ItemsDeleteCatalogRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ItemsDeleteCatalogRequestBuilder {
    id: Option<String>,
}

impl ItemsDeleteCatalogRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ItemsDeleteCatalogRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ItemsDeleteCatalogRequestBuilder::id)
    pub fn build(self) -> Result<ItemsDeleteCatalogRequest, BuildError> {
        Ok(ItemsDeleteCatalogRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
