pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MergePartnersResponseMovedItem {
    #[serde(default)]
    pub table: String,
    #[serde(default)]
    pub column: String,
    #[serde(default)]
    pub rows: i64,
}

impl MergePartnersResponseMovedItem {
    pub fn builder() -> MergePartnersResponseMovedItemBuilder {
        <MergePartnersResponseMovedItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MergePartnersResponseMovedItemBuilder {
    table: Option<String>,
    column: Option<String>,
    rows: Option<i64>,
}

impl MergePartnersResponseMovedItemBuilder {
    pub fn table(mut self, value: impl Into<String>) -> Self {
        self.table = Some(value.into());
        self
    }

    pub fn column(mut self, value: impl Into<String>) -> Self {
        self.column = Some(value.into());
        self
    }

    pub fn rows(mut self, value: i64) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MergePartnersResponseMovedItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`table`](MergePartnersResponseMovedItemBuilder::table)
    /// - [`column`](MergePartnersResponseMovedItemBuilder::column)
    /// - [`rows`](MergePartnersResponseMovedItemBuilder::rows)
    pub fn build(self) -> Result<MergePartnersResponseMovedItem, BuildError> {
        Ok(MergePartnersResponseMovedItem {
            table: self
                .table
                .ok_or_else(|| BuildError::missing_field("table"))?,
            column: self
                .column
                .ok_or_else(|| BuildError::missing_field("column"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
