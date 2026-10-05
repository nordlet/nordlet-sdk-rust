pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReferralGetAccountRequest {}

impl ReferralGetAccountRequest {
    pub fn builder() -> ReferralGetAccountRequestBuilder {
        <ReferralGetAccountRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReferralGetAccountRequestBuilder {}

impl ReferralGetAccountRequestBuilder {
    /// Consumes the builder and constructs a [`ReferralGetAccountRequest`].
    pub fn build(self) -> Result<ReferralGetAccountRequest, BuildError> {
        Ok(ReferralGetAccountRequest {})
    }
}
