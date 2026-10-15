pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ImportTemplatesListBuiltinBankRequest {}

impl ImportTemplatesListBuiltinBankRequest {
    pub fn builder() -> ImportTemplatesListBuiltinBankRequestBuilder {
        <ImportTemplatesListBuiltinBankRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ImportTemplatesListBuiltinBankRequestBuilder {}

impl ImportTemplatesListBuiltinBankRequestBuilder {
    /// Consumes the builder and constructs a [`ImportTemplatesListBuiltinBankRequest`].
    pub fn build(self) -> Result<ImportTemplatesListBuiltinBankRequest, BuildError> {
        Ok(ImportTemplatesListBuiltinBankRequest {})
    }
}
