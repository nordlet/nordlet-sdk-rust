pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ImportTemplatesListBuiltinBankResponse {
    #[serde(default)]
    pub rows: Vec<ImportTemplatesListBuiltinBankResponseRowsItem>,
}

impl ImportTemplatesListBuiltinBankResponse {
    pub fn builder() -> ImportTemplatesListBuiltinBankResponseBuilder {
        <ImportTemplatesListBuiltinBankResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ImportTemplatesListBuiltinBankResponseBuilder {
    rows: Option<Vec<ImportTemplatesListBuiltinBankResponseRowsItem>>,
}

impl ImportTemplatesListBuiltinBankResponseBuilder {
    pub fn rows(mut self, value: Vec<ImportTemplatesListBuiltinBankResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ImportTemplatesListBuiltinBankResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](ImportTemplatesListBuiltinBankResponseBuilder::rows)
    pub fn build(self) -> Result<ImportTemplatesListBuiltinBankResponse, BuildError> {
        Ok(ImportTemplatesListBuiltinBankResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
