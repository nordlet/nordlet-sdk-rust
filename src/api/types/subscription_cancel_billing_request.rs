pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubscriptionCancelBillingRequest {}

impl SubscriptionCancelBillingRequest {
    pub fn builder() -> SubscriptionCancelBillingRequestBuilder {
        <SubscriptionCancelBillingRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscriptionCancelBillingRequestBuilder {}

impl SubscriptionCancelBillingRequestBuilder {
    /// Consumes the builder and constructs a [`SubscriptionCancelBillingRequest`].
    pub fn build(self) -> Result<SubscriptionCancelBillingRequest, BuildError> {
        Ok(SubscriptionCancelBillingRequest {})
    }
}
