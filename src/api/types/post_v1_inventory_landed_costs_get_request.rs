pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1InventoryLandedCostsGetRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1InventoryLandedCostsGetRequest {
    pub fn builder() -> PostV1InventoryLandedCostsGetRequestBuilder {
        <PostV1InventoryLandedCostsGetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1InventoryLandedCostsGetRequestBuilder {
    id: Option<String>,
}

impl PostV1InventoryLandedCostsGetRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1InventoryLandedCostsGetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1InventoryLandedCostsGetRequestBuilder::id)
    pub fn build(self) -> Result<PostV1InventoryLandedCostsGetRequest, BuildError> {
        Ok(PostV1InventoryLandedCostsGetRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
