pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PaymentsListSalesResponse {
    #[serde(default)]
    pub rows: Vec<PaymentsListSalesResponseRowsItem>,
    #[serde(default)]
    pub page: i64,
    #[serde(rename = "pageSize")]
    #[serde(default)]
    pub page_size: i64,
    #[serde(default)]
    pub total: i64,
    #[serde(rename = "totalsByCurrency")]
    #[serde(default)]
    pub totals_by_currency: HashMap<String, String>,
}

impl PaymentsListSalesResponse {
    pub fn builder() -> PaymentsListSalesResponseBuilder {
        <PaymentsListSalesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentsListSalesResponseBuilder {
    rows: Option<Vec<PaymentsListSalesResponseRowsItem>>,
    page: Option<i64>,
    page_size: Option<i64>,
    total: Option<i64>,
    totals_by_currency: Option<HashMap<String, String>>,
}

impl PaymentsListSalesResponseBuilder {
    pub fn rows(mut self, value: Vec<PaymentsListSalesResponseRowsItem>) -> Self {
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

    pub fn totals_by_currency(mut self, value: HashMap<String, String>) -> Self {
        self.totals_by_currency = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentsListSalesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PaymentsListSalesResponseBuilder::rows)
    /// - [`page`](PaymentsListSalesResponseBuilder::page)
    /// - [`page_size`](PaymentsListSalesResponseBuilder::page_size)
    /// - [`total`](PaymentsListSalesResponseBuilder::total)
    /// - [`totals_by_currency`](PaymentsListSalesResponseBuilder::totals_by_currency)
    pub fn build(self) -> Result<PaymentsListSalesResponse, BuildError> {
        Ok(PaymentsListSalesResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
            page: self.page.ok_or_else(|| BuildError::missing_field("page"))?,
            page_size: self
                .page_size
                .ok_or_else(|| BuildError::missing_field("page_size"))?,
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
            totals_by_currency: self
                .totals_by_currency
                .ok_or_else(|| BuildError::missing_field("totals_by_currency"))?,
        })
    }
}
