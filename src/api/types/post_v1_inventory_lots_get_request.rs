pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1InventoryLotsGetRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1InventoryLotsGetRequest {
    pub fn builder() -> PostV1InventoryLotsGetRequestBuilder {
        <PostV1InventoryLotsGetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1InventoryLotsGetRequestBuilder {
    id: Option<String>,
}

impl PostV1InventoryLotsGetRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1InventoryLotsGetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1InventoryLotsGetRequestBuilder::id)
    pub fn build(self) -> Result<PostV1InventoryLotsGetRequest, BuildError> {
        Ok(PostV1InventoryLotsGetRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
