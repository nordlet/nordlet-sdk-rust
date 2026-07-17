pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerPostingRulesListRequest {}

impl PostV1LedgerPostingRulesListRequest {
    pub fn builder() -> PostV1LedgerPostingRulesListRequestBuilder {
        <PostV1LedgerPostingRulesListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerPostingRulesListRequestBuilder {}

impl PostV1LedgerPostingRulesListRequestBuilder {
    /// Consumes the builder and constructs a [`PostV1LedgerPostingRulesListRequest`].
    pub fn build(self) -> Result<PostV1LedgerPostingRulesListRequest, BuildError> {
        Ok(PostV1LedgerPostingRulesListRequest {})
    }
}
