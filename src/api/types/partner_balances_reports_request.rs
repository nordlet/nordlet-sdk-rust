pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PartnerBalancesReportsRequest {}

impl PartnerBalancesReportsRequest {
    pub fn builder() -> PartnerBalancesReportsRequestBuilder {
        <PartnerBalancesReportsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PartnerBalancesReportsRequestBuilder {}

impl PartnerBalancesReportsRequestBuilder {
    /// Consumes the builder and constructs a [`PartnerBalancesReportsRequest`].
    pub fn build(self) -> Result<PartnerBalancesReportsRequest, BuildError> {
        Ok(PartnerBalancesReportsRequest {})
    }
}
