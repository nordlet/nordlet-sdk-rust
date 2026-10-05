pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ActsListSalesResponse {
    #[serde(default)]
    pub rows: Vec<ActsListSalesResponseRowsItem>,
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

impl ActsListSalesResponse {
    pub fn builder() -> ActsListSalesResponseBuilder {
        <ActsListSalesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ActsListSalesResponseBuilder {
    rows: Option<Vec<ActsListSalesResponseRowsItem>>,
    page: Option<i64>,
    page_size: Option<i64>,
    total: Option<i64>,
    totals: Option<HashMap<String, String>>,
}

impl ActsListSalesResponseBuilder {
    pub fn rows(mut self, value: Vec<ActsListSalesResponseRowsItem>) -> Self {
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

    /// Consumes the builder and constructs a [`ActsListSalesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](ActsListSalesResponseBuilder::rows)
    /// - [`page`](ActsListSalesResponseBuilder::page)
    /// - [`page_size`](ActsListSalesResponseBuilder::page_size)
    /// - [`total`](ActsListSalesResponseBuilder::total)
    pub fn build(self) -> Result<ActsListSalesResponse, BuildError> {
        Ok(ActsListSalesResponse {
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
