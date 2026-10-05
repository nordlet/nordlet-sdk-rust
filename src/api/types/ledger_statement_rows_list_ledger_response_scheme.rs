pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StatementRowsListLedgerResponseScheme {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub country: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub rows: Vec<StatementRowsListLedgerResponseSchemeRowsItem>,
}

impl StatementRowsListLedgerResponseScheme {
    pub fn builder() -> StatementRowsListLedgerResponseSchemeBuilder {
        <StatementRowsListLedgerResponseSchemeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StatementRowsListLedgerResponseSchemeBuilder {
    key: Option<String>,
    country: Option<String>,
    title: Option<String>,
    source: Option<String>,
    rows: Option<Vec<StatementRowsListLedgerResponseSchemeRowsItem>>,
}

impl StatementRowsListLedgerResponseSchemeBuilder {
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

    pub fn rows(mut self, value: Vec<StatementRowsListLedgerResponseSchemeRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StatementRowsListLedgerResponseScheme`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](StatementRowsListLedgerResponseSchemeBuilder::key)
    /// - [`country`](StatementRowsListLedgerResponseSchemeBuilder::country)
    /// - [`title`](StatementRowsListLedgerResponseSchemeBuilder::title)
    /// - [`source`](StatementRowsListLedgerResponseSchemeBuilder::source)
    /// - [`rows`](StatementRowsListLedgerResponseSchemeBuilder::rows)
    pub fn build(self) -> Result<StatementRowsListLedgerResponseScheme, BuildError> {
        Ok(StatementRowsListLedgerResponseScheme {
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
