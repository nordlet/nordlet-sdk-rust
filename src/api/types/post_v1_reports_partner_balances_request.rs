pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsPartnerBalancesRequest {}

impl PostV1ReportsPartnerBalancesRequest {
    pub fn builder() -> PostV1ReportsPartnerBalancesRequestBuilder {
        <PostV1ReportsPartnerBalancesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsPartnerBalancesRequestBuilder {}

impl PostV1ReportsPartnerBalancesRequestBuilder {
    /// Consumes the builder and constructs a [`PostV1ReportsPartnerBalancesRequest`].
    pub fn build(self) -> Result<PostV1ReportsPartnerBalancesRequest, BuildError> {
        Ok(PostV1ReportsPartnerBalancesRequest {})
    }
}
