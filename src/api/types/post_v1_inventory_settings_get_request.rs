pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1InventorySettingsGetRequest {}

impl PostV1InventorySettingsGetRequest {
    pub fn builder() -> PostV1InventorySettingsGetRequestBuilder {
        <PostV1InventorySettingsGetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1InventorySettingsGetRequestBuilder {}

impl PostV1InventorySettingsGetRequestBuilder {
    /// Consumes the builder and constructs a [`PostV1InventorySettingsGetRequest`].
    pub fn build(self) -> Result<PostV1InventorySettingsGetRequest, BuildError> {
        Ok(PostV1InventorySettingsGetRequest {})
    }
}
