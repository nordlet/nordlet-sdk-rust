pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BillingAccountGetRequest {}

impl PostV1BillingAccountGetRequest {
    pub fn builder() -> PostV1BillingAccountGetRequestBuilder {
        <PostV1BillingAccountGetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BillingAccountGetRequestBuilder {}

impl PostV1BillingAccountGetRequestBuilder {
    /// Consumes the builder and constructs a [`PostV1BillingAccountGetRequest`].
    pub fn build(self) -> Result<PostV1BillingAccountGetRequest, BuildError> {
        Ok(PostV1BillingAccountGetRequest {})
    }
}
