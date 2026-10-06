pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ReportsListPosResponse {
    #[serde(default)]
    pub rows: Vec<ReportsListPosResponseRowsItem>,
    #[serde(default)]
    pub page: i64,
    #[serde(rename = "pageSize")]
    #[serde(default)]
    pub page_size: i64,
    #[serde(default)]
    pub total: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totals: Option<HashMap<String, String>>,
    /// The requested totals split by currency code, present when the listed records carry a currency
    #[serde(rename = "totalsByCurrency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totals_by_currency: Option<HashMap<String, HashMap<String, String>>>,
}

impl ReportsListPosResponse {
    pub fn builder() -> ReportsListPosResponseBuilder {
        <ReportsListPosResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReportsListPosResponseBuilder {
    rows: Option<Vec<ReportsListPosResponseRowsItem>>,
    page: Option<i64>,
    page_size: Option<i64>,
    total: Option<i64>,
    totals: Option<HashMap<String, String>>,
    totals_by_currency: Option<HashMap<String, HashMap<String, String>>>,
}

impl ReportsListPosResponseBuilder {
    pub fn rows(mut self, value: Vec<ReportsListPosResponseRowsItem>) -> Self {
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

    pub fn totals_by_currency(mut self, value: HashMap<String, HashMap<String, String>>) -> Self {
        self.totals_by_currency = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReportsListPosResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](ReportsListPosResponseBuilder::rows)
    /// - [`page`](ReportsListPosResponseBuilder::page)
    /// - [`page_size`](ReportsListPosResponseBuilder::page_size)
    /// - [`total`](ReportsListPosResponseBuilder::total)
    pub fn build(self) -> Result<ReportsListPosResponse, BuildError> {
        Ok(ReportsListPosResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
            page: self.page.ok_or_else(|| BuildError::missing_field("page"))?,
            page_size: self
                .page_size
                .ok_or_else(|| BuildError::missing_field("page_size"))?,
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
            totals: self.totals,
            totals_by_currency: self.totals_by_currency,
        })
    }
}
