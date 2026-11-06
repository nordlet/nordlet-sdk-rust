pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FreeSandboxStartBillingRequest {}

impl FreeSandboxStartBillingRequest {
    pub fn builder() -> FreeSandboxStartBillingRequestBuilder {
        <FreeSandboxStartBillingRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FreeSandboxStartBillingRequestBuilder {}

impl FreeSandboxStartBillingRequestBuilder {
    /// Consumes the builder and constructs a [`FreeSandboxStartBillingRequest`].
    pub fn build(self) -> Result<FreeSandboxStartBillingRequest, BuildError> {
        Ok(FreeSandboxStartBillingRequest {})
    }
}
