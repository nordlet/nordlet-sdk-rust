pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BankMatchRulesListRequest {}

impl PostV1BankMatchRulesListRequest {
    pub fn builder() -> PostV1BankMatchRulesListRequestBuilder {
        <PostV1BankMatchRulesListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankMatchRulesListRequestBuilder {}

impl PostV1BankMatchRulesListRequestBuilder {
    /// Consumes the builder and constructs a [`PostV1BankMatchRulesListRequest`].
    pub fn build(self) -> Result<PostV1BankMatchRulesListRequest, BuildError> {
        Ok(PostV1BankMatchRulesListRequest {})
    }
}
