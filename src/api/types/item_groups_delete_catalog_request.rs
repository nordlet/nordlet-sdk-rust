pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ItemGroupsDeleteCatalogRequest {
    #[serde(default)]
    pub id: String,
}

impl ItemGroupsDeleteCatalogRequest {
    pub fn builder() -> ItemGroupsDeleteCatalogRequestBuilder {
        <ItemGroupsDeleteCatalogRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ItemGroupsDeleteCatalogRequestBuilder {
    id: Option<String>,
}

impl ItemGroupsDeleteCatalogRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ItemGroupsDeleteCatalogRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ItemGroupsDeleteCatalogRequestBuilder::id)
    pub fn build(self) -> Result<ItemGroupsDeleteCatalogRequest, BuildError> {
        Ok(ItemGroupsDeleteCatalogRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
