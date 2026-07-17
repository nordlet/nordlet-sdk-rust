pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsGeneralJournalResponse {
    #[serde(default)]
    pub total: i64,
    #[serde(default)]
    pub page: i64,
    #[serde(rename = "pageSize")]
    #[serde(default)]
    pub page_size: i64,
    #[serde(default)]
    pub rows: Vec<PostV1ReportsGeneralJournalResponseRowsItem>,
}

impl PostV1ReportsGeneralJournalResponse {
    pub fn builder() -> PostV1ReportsGeneralJournalResponseBuilder {
        <PostV1ReportsGeneralJournalResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsGeneralJournalResponseBuilder {
    total: Option<i64>,
    page: Option<i64>,
    page_size: Option<i64>,
    rows: Option<Vec<PostV1ReportsGeneralJournalResponseRowsItem>>,
}

impl PostV1ReportsGeneralJournalResponseBuilder {
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

    pub fn rows(mut self, value: Vec<PostV1ReportsGeneralJournalResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsGeneralJournalResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`total`](PostV1ReportsGeneralJournalResponseBuilder::total)
    /// - [`page`](PostV1ReportsGeneralJournalResponseBuilder::page)
    /// - [`page_size`](PostV1ReportsGeneralJournalResponseBuilder::page_size)
    /// - [`rows`](PostV1ReportsGeneralJournalResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1ReportsGeneralJournalResponse, BuildError> {
        Ok(PostV1ReportsGeneralJournalResponse {
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
