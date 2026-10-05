pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TableSettingsListAccountResponse {
    #[serde(default)]
    pub rows: Vec<TableSettingsListAccountResponseRowsItem>,
}

impl TableSettingsListAccountResponse {
    pub fn builder() -> TableSettingsListAccountResponseBuilder {
        <TableSettingsListAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TableSettingsListAccountResponseBuilder {
    rows: Option<Vec<TableSettingsListAccountResponseRowsItem>>,
}

impl TableSettingsListAccountResponseBuilder {
    pub fn rows(mut self, value: Vec<TableSettingsListAccountResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TableSettingsListAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](TableSettingsListAccountResponseBuilder::rows)
    pub fn build(self) -> Result<TableSettingsListAccountResponse, BuildError> {
        Ok(TableSettingsListAccountResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
