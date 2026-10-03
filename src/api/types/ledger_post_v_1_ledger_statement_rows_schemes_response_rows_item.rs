pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerStatementRowsSchemesResponseRowsItem {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub country: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub rows: Vec<PostV1LedgerStatementRowsSchemesResponseRowsItemRowsItem>,
}

impl PostV1LedgerStatementRowsSchemesResponseRowsItem {
    pub fn builder() -> PostV1LedgerStatementRowsSchemesResponseRowsItemBuilder {
        <PostV1LedgerStatementRowsSchemesResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerStatementRowsSchemesResponseRowsItemBuilder {
    key: Option<String>,
    country: Option<String>,
    title: Option<String>,
    source: Option<String>,
    rows: Option<Vec<PostV1LedgerStatementRowsSchemesResponseRowsItemRowsItem>>,
}

impl PostV1LedgerStatementRowsSchemesResponseRowsItemBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn rows(
        mut self,
        value: Vec<PostV1LedgerStatementRowsSchemesResponseRowsItemRowsItem>,
    ) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerStatementRowsSchemesResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](PostV1LedgerStatementRowsSchemesResponseRowsItemBuilder::key)
    /// - [`country`](PostV1LedgerStatementRowsSchemesResponseRowsItemBuilder::country)
    /// - [`title`](PostV1LedgerStatementRowsSchemesResponseRowsItemBuilder::title)
    /// - [`source`](PostV1LedgerStatementRowsSchemesResponseRowsItemBuilder::source)
    /// - [`rows`](PostV1LedgerStatementRowsSchemesResponseRowsItemBuilder::rows)
    pub fn build(self) -> Result<PostV1LedgerStatementRowsSchemesResponseRowsItem, BuildError> {
        Ok(PostV1LedgerStatementRowsSchemesResponseRowsItem {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            country: self
                .country
                .ok_or_else(|| BuildError::missing_field("country"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
