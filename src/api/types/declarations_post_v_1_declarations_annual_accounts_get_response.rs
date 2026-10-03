pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsAnnualAccountsGetResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval: Option<PostV1DeclarationsAnnualAccountsGetResponseApproval>,
}

impl PostV1DeclarationsAnnualAccountsGetResponse {
    pub fn builder() -> PostV1DeclarationsAnnualAccountsGetResponseBuilder {
        <PostV1DeclarationsAnnualAccountsGetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsAnnualAccountsGetResponseBuilder {
    approval: Option<PostV1DeclarationsAnnualAccountsGetResponseApproval>,
}

impl PostV1DeclarationsAnnualAccountsGetResponseBuilder {
    pub fn approval(mut self, value: PostV1DeclarationsAnnualAccountsGetResponseApproval) -> Self {
        self.approval = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsAnnualAccountsGetResponse`].
    pub fn build(self) -> Result<PostV1DeclarationsAnnualAccountsGetResponse, BuildError> {
        Ok(PostV1DeclarationsAnnualAccountsGetResponse {
            approval: self.approval,
        })
    }
}
