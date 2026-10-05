pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ReceiptsListPurchasesResponse {
    #[serde(default)]
    pub rows: Vec<ReceiptsListPurchasesResponseRowsItem>,
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

impl ReceiptsListPurchasesResponse {
    pub fn builder() -> ReceiptsListPurchasesResponseBuilder {
        <ReceiptsListPurchasesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceiptsListPurchasesResponseBuilder {
    rows: Option<Vec<ReceiptsListPurchasesResponseRowsItem>>,
    page: Option<i64>,
    page_size: Option<i64>,
    total: Option<i64>,
    totals: Option<HashMap<String, String>>,
}

impl ReceiptsListPurchasesResponseBuilder {
    pub fn rows(mut self, value: Vec<ReceiptsListPurchasesResponseRowsItem>) -> Self {
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

    /// Consumes the builder and constructs a [`ReceiptsListPurchasesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](ReceiptsListPurchasesResponseBuilder::rows)
    /// - [`page`](ReceiptsListPurchasesResponseBuilder::page)
    /// - [`page_size`](ReceiptsListPurchasesResponseBuilder::page_size)
    /// - [`total`](ReceiptsListPurchasesResponseBuilder::total)
    pub fn build(self) -> Result<ReceiptsListPurchasesResponse, BuildError> {
        Ok(ReceiptsListPurchasesResponse {
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
