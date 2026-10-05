pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExportAccountRequest {}

impl ExportAccountRequest {
    pub fn builder() -> ExportAccountRequestBuilder {
        <ExportAccountRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExportAccountRequestBuilder {}

impl ExportAccountRequestBuilder {
    /// Consumes the builder and constructs a [`ExportAccountRequest`].
    pub fn build(self) -> Result<ExportAccountRequest, BuildError> {
        Ok(ExportAccountRequest {})
    }
}
