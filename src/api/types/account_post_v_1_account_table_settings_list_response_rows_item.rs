pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountTableSettingsListResponseRowsItem {
    #[serde(rename = "tableKey")]
    #[serde(default)]
    pub table_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<String>>,
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

impl PostV1AccountTableSettingsListResponseRowsItem {
    pub fn builder() -> PostV1AccountTableSettingsListResponseRowsItemBuilder {
        <PostV1AccountTableSettingsListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountTableSettingsListResponseRowsItemBuilder {
    table_key: Option<String>,
    columns: Option<Vec<String>>,
    page_size: Option<i64>,
}

impl PostV1AccountTableSettingsListResponseRowsItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1AccountTableSettingsListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`table_key`](PostV1AccountTableSettingsListResponseRowsItemBuilder::table_key)
    pub fn build(self) -> Result<PostV1AccountTableSettingsListResponseRowsItem, BuildError> {
        Ok(PostV1AccountTableSettingsListResponseRowsItem {
            table_key: self
                .table_key
                .ok_or_else(|| BuildError::missing_field("table_key"))?,
            columns: self.columns,
            page_size: self.page_size,
        })
    }
}
