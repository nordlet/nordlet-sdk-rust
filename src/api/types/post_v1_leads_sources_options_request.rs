pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LeadsSourcesOptionsRequest {}

impl PostV1LeadsSourcesOptionsRequest {
    pub fn builder() -> PostV1LeadsSourcesOptionsRequestBuilder {
        <PostV1LeadsSourcesOptionsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LeadsSourcesOptionsRequestBuilder {}

impl PostV1LeadsSourcesOptionsRequestBuilder {
    /// Consumes the builder and constructs a [`PostV1LeadsSourcesOptionsRequest`].
    pub fn build(self) -> Result<PostV1LeadsSourcesOptionsRequest, BuildError> {
        Ok(PostV1LeadsSourcesOptionsRequest {})
    }
}
