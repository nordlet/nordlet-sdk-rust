pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountExportRequest {}

impl PostV1AccountExportRequest {
    pub fn builder() -> PostV1AccountExportRequestBuilder {
        <PostV1AccountExportRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountExportRequestBuilder {}

impl PostV1AccountExportRequestBuilder {
    /// Consumes the builder and constructs a [`PostV1AccountExportRequest`].
    pub fn build(self) -> Result<PostV1AccountExportRequest, BuildError> {
        Ok(PostV1AccountExportRequest {})
    }
}
