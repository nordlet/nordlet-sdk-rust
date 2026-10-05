pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TableSettingsListAccountRequest {}

impl TableSettingsListAccountRequest {
    pub fn builder() -> TableSettingsListAccountRequestBuilder {
        <TableSettingsListAccountRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TableSettingsListAccountRequestBuilder {}

impl TableSettingsListAccountRequestBuilder {
    /// Consumes the builder and constructs a [`TableSettingsListAccountRequest`].
    pub fn build(self) -> Result<TableSettingsListAccountRequest, BuildError> {
        Ok(TableSettingsListAccountRequest {})
    }
}
