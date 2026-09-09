pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountTableSettingsListRequest {}

impl PostV1AccountTableSettingsListRequest {
    pub fn builder() -> PostV1AccountTableSettingsListRequestBuilder {
        <PostV1AccountTableSettingsListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountTableSettingsListRequestBuilder {}

impl PostV1AccountTableSettingsListRequestBuilder {
    /// Consumes the builder and constructs a [`PostV1AccountTableSettingsListRequest`].
    pub fn build(self) -> Result<PostV1AccountTableSettingsListRequest, BuildError> {
        Ok(PostV1AccountTableSettingsListRequest {})
    }
}
