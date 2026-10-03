pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsAutomationListRequest {}

impl PostV1DeclarationsAutomationListRequest {
    pub fn builder() -> PostV1DeclarationsAutomationListRequestBuilder {
        <PostV1DeclarationsAutomationListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsAutomationListRequestBuilder {}

impl PostV1DeclarationsAutomationListRequestBuilder {
    /// Consumes the builder and constructs a [`PostV1DeclarationsAutomationListRequest`].
    pub fn build(self) -> Result<PostV1DeclarationsAutomationListRequest, BuildError> {
        Ok(PostV1DeclarationsAutomationListRequest {})
    }
}
