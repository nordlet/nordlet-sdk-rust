pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TableSettingsGetAccountRequest {
    #[serde(rename = "tableKey")]
    #[serde(default)]
    pub table_key: String,
}

impl TableSettingsGetAccountRequest {
    pub fn builder() -> TableSettingsGetAccountRequestBuilder {
        <TableSettingsGetAccountRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TableSettingsGetAccountRequestBuilder {
    table_key: Option<String>,
}

impl TableSettingsGetAccountRequestBuilder {
    pub fn table_key(mut self, value: impl Into<String>) -> Self {
        self.table_key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TableSettingsGetAccountRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`table_key`](TableSettingsGetAccountRequestBuilder::table_key)
    pub fn build(self) -> Result<TableSettingsGetAccountRequest, BuildError> {
        Ok(TableSettingsGetAccountRequest {
            table_key: self
                .table_key
                .ok_or_else(|| BuildError::missing_field("table_key"))?,
        })
    }
}
