pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MaintenanceListProductionResponse {
    #[serde(default)]
    pub rows: Vec<MaintenanceListProductionResponseRowsItem>,
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

impl MaintenanceListProductionResponse {
    pub fn builder() -> MaintenanceListProductionResponseBuilder {
        <MaintenanceListProductionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MaintenanceListProductionResponseBuilder {
    rows: Option<Vec<MaintenanceListProductionResponseRowsItem>>,
    page: Option<i64>,
    page_size: Option<i64>,
    total: Option<i64>,
    totals: Option<HashMap<String, String>>,
}

impl MaintenanceListProductionResponseBuilder {
    pub fn rows(mut self, value: Vec<MaintenanceListProductionResponseRowsItem>) -> Self {
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

    /// Consumes the builder and constructs a [`MaintenanceListProductionResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](MaintenanceListProductionResponseBuilder::rows)
    /// - [`page`](MaintenanceListProductionResponseBuilder::page)
    /// - [`page_size`](MaintenanceListProductionResponseBuilder::page_size)
    /// - [`total`](MaintenanceListProductionResponseBuilder::total)
    pub fn build(self) -> Result<MaintenanceListProductionResponse, BuildError> {
        Ok(MaintenanceListProductionResponse {
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
