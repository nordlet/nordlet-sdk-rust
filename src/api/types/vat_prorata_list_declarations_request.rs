pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VatProrataListDeclarationsRequest {}

impl VatProrataListDeclarationsRequest {
    pub fn builder() -> VatProrataListDeclarationsRequestBuilder {
        <VatProrataListDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VatProrataListDeclarationsRequestBuilder {}

impl VatProrataListDeclarationsRequestBuilder {
    /// Consumes the builder and constructs a [`VatProrataListDeclarationsRequest`].
    pub fn build(self) -> Result<VatProrataListDeclarationsRequest, BuildError> {
        Ok(VatProrataListDeclarationsRequest {})
    }
}
