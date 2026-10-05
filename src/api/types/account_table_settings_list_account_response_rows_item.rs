pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TableSettingsListAccountResponseRowsItem {
    #[serde(rename = "tableKey")]
    #[serde(default)]
    pub table_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<String>>,
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

impl TableSettingsListAccountResponseRowsItem {
    pub fn builder() -> TableSettingsListAccountResponseRowsItemBuilder {
        <TableSettingsListAccountResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TableSettingsListAccountResponseRowsItemBuilder {
    table_key: Option<String>,
    columns: Option<Vec<String>>,
    page_size: Option<i64>,
}

impl TableSettingsListAccountResponseRowsItemBuilder {
    pub fn table_key(mut self, value: impl Into<String>) -> Self {
        self.table_key = Some(value.into());
        self
    }

    pub fn columns(mut self, value: Vec<String>) -> Self {
        self.columns = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TableSettingsListAccountResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`table_key`](TableSettingsListAccountResponseRowsItemBuilder::table_key)
    pub fn build(self) -> Result<TableSettingsListAccountResponseRowsItem, BuildError> {
        Ok(TableSettingsListAccountResponseRowsItem {
            table_key: self
                .table_key
                .ok_or_else(|| BuildError::missing_field("table_key"))?,
            columns: self.columns,
            page_size: self.page_size,
        })
    }
}
