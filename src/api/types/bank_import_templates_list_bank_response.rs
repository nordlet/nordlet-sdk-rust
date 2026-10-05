pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ImportTemplatesListBankResponse {
    #[serde(default)]
    pub rows: Vec<ImportTemplatesListBankResponseRowsItem>,
    #[serde(default)]
    pub page: i64,
    #[serde(rename = "pageSize")]
    #[serde(default)]
    pub page_size: i64,
    #[serde(default)]
    pub total: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totals: Option<HashMap<String, String>>,
}

impl ImportTemplatesListBankResponse {
    pub fn builder() -> ImportTemplatesListBankResponseBuilder {
        <ImportTemplatesListBankResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ImportTemplatesListBankResponseBuilder {
    rows: Option<Vec<ImportTemplatesListBankResponseRowsItem>>,
    page: Option<i64>,
    page_size: Option<i64>,
    total: Option<i64>,
    totals: Option<HashMap<String, String>>,
}

impl ImportTemplatesListBankResponseBuilder {
    pub fn rows(mut self, value: Vec<ImportTemplatesListBankResponseRowsItem>) -> Self {
        self.rows = Some(value);
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

    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
        self
    }

    pub fn totals(mut self, value: HashMap<String, String>) -> Self {
        self.totals = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ImportTemplatesListBankResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](ImportTemplatesListBankResponseBuilder::rows)
    /// - [`page`](ImportTemplatesListBankResponseBuilder::page)
    /// - [`page_size`](ImportTemplatesListBankResponseBuilder::page_size)
    /// - [`total`](ImportTemplatesListBankResponseBuilder::total)
    pub fn build(self) -> Result<ImportTemplatesListBankResponse, BuildError> {
        Ok(ImportTemplatesListBankResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
            page: self.page.ok_or_else(|| BuildError::missing_field("page"))?,
            page_size: self
                .page_size
                .ok_or_else(|| BuildError::missing_field("page_size"))?,
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
            totals: self.totals,
        })
    }
}
