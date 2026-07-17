pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerAccountsApplyTemplateRequest {}

impl PostV1LedgerAccountsApplyTemplateRequest {
    pub fn builder() -> PostV1LedgerAccountsApplyTemplateRequestBuilder {
        <PostV1LedgerAccountsApplyTemplateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerAccountsApplyTemplateRequestBuilder {}

impl PostV1LedgerAccountsApplyTemplateRequestBuilder {
    /// Consumes the builder and constructs a [`PostV1LedgerAccountsApplyTemplateRequest`].
    pub fn build(self) -> Result<PostV1LedgerAccountsApplyTemplateRequest, BuildError> {
        Ok(PostV1LedgerAccountsApplyTemplateRequest {})
    }
}
