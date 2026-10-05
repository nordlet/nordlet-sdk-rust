pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CompaniesProfileAccountRequest {}

impl CompaniesProfileAccountRequest {
    pub fn builder() -> CompaniesProfileAccountRequestBuilder {
        <CompaniesProfileAccountRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompaniesProfileAccountRequestBuilder {}

impl CompaniesProfileAccountRequestBuilder {
    /// Consumes the builder and constructs a [`CompaniesProfileAccountRequest`].
    pub fn build(self) -> Result<CompaniesProfileAccountRequest, BuildError> {
        Ok(CompaniesProfileAccountRequest {})
    }
}
