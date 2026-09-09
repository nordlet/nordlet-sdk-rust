pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountReferralGetRequest {}

impl PostV1AccountReferralGetRequest {
    pub fn builder() -> PostV1AccountReferralGetRequestBuilder {
        <PostV1AccountReferralGetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountReferralGetRequestBuilder {}

impl PostV1AccountReferralGetRequestBuilder {
    /// Consumes the builder and constructs a [`PostV1AccountReferralGetRequest`].
    pub fn build(self) -> Result<PostV1AccountReferralGetRequest, BuildError> {
        Ok(PostV1AccountReferralGetRequest {})
    }
}
