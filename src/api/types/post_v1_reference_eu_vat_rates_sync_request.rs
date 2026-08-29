pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceEuVatRatesSyncRequest {}

impl PostV1ReferenceEuVatRatesSyncRequest {
    pub fn builder() -> PostV1ReferenceEuVatRatesSyncRequestBuilder {
        <PostV1ReferenceEuVatRatesSyncRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceEuVatRatesSyncRequestBuilder {}

impl PostV1ReferenceEuVatRatesSyncRequestBuilder {
    /// Consumes the builder and constructs a [`PostV1ReferenceEuVatRatesSyncRequest`].
    pub fn build(self) -> Result<PostV1ReferenceEuVatRatesSyncRequest, BuildError> {
        Ok(PostV1ReferenceEuVatRatesSyncRequest {})
    }
}
