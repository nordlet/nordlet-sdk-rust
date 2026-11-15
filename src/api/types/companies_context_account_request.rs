pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CompaniesContextAccountRequest {}

impl CompaniesContextAccountRequest {
    pub fn builder() -> CompaniesContextAccountRequestBuilder {
        <CompaniesContextAccountRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompaniesContextAccountRequestBuilder {}

impl CompaniesContextAccountRequestBuilder {
    /// Consumes the builder and constructs a [`CompaniesContextAccountRequest`].
    pub fn build(self) -> Result<CompaniesContextAccountRequest, BuildError> {
        Ok(CompaniesContextAccountRequest {})
    }
}
