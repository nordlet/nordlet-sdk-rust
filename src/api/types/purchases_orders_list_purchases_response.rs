pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OrdersListPurchasesResponse {
    #[serde(default)]
    pub rows: Vec<OrdersListPurchasesResponseRowsItem>,
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

impl OrdersListPurchasesResponse {
    pub fn builder() -> OrdersListPurchasesResponseBuilder {
        <OrdersListPurchasesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrdersListPurchasesResponseBuilder {
    rows: Option<Vec<OrdersListPurchasesResponseRowsItem>>,
    page: Option<i64>,
    page_size: Option<i64>,
    total: Option<i64>,
    totals: Option<HashMap<String, String>>,
}

impl OrdersListPurchasesResponseBuilder {
    pub fn rows(mut self, value: Vec<OrdersListPurchasesResponseRowsItem>) -> Self {
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

    /// Consumes the builder and constructs a [`OrdersListPurchasesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](OrdersListPurchasesResponseBuilder::rows)
    /// - [`page`](OrdersListPurchasesResponseBuilder::page)
    /// - [`page_size`](OrdersListPurchasesResponseBuilder::page_size)
    /// - [`total`](OrdersListPurchasesResponseBuilder::total)
    pub fn build(self) -> Result<OrdersListPurchasesResponse, BuildError> {
        Ok(OrdersListPurchasesResponse {
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
