pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BrandGetAccountRequest {}

impl BrandGetAccountRequest {
    pub fn builder() -> BrandGetAccountRequestBuilder {
        <BrandGetAccountRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BrandGetAccountRequestBuilder {}

impl BrandGetAccountRequestBuilder {
    /// Consumes the builder and constructs a [`BrandGetAccountRequest`].
    pub fn build(self) -> Result<BrandGetAccountRequest, BuildError> {
        Ok(BrandGetAccountRequest {})
    }
}
