pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GeneralJournalReportsResponse {
    #[serde(default)]
    pub total: i64,
    #[serde(default)]
    pub page: i64,
    #[serde(rename = "pageSize")]
    #[serde(default)]
    pub page_size: i64,
    #[serde(default)]
    pub rows: Vec<GeneralJournalReportsResponseRowsItem>,
}

impl GeneralJournalReportsResponse {
    pub fn builder() -> GeneralJournalReportsResponseBuilder {
        <GeneralJournalReportsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GeneralJournalReportsResponseBuilder {
    total: Option<i64>,
    page: Option<i64>,
    page_size: Option<i64>,
    rows: Option<Vec<GeneralJournalReportsResponseRowsItem>>,
}

impl GeneralJournalReportsResponseBuilder {
    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn rows(mut self, value: Vec<GeneralJournalReportsResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GeneralJournalReportsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`total`](GeneralJournalReportsResponseBuilder::total)
    /// - [`page`](GeneralJournalReportsResponseBuilder::page)
    /// - [`page_size`](GeneralJournalReportsResponseBuilder::page_size)
    /// - [`rows`](GeneralJournalReportsResponseBuilder::rows)
    pub fn build(self) -> Result<GeneralJournalReportsResponse, BuildError> {
        Ok(GeneralJournalReportsResponse {
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
            page: self.page.ok_or_else(|| BuildError::missing_field("page"))?,
            page_size: self
                .page_size
                .ok_or_else(|| BuildError::missing_field("page_size"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
