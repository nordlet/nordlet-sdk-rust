pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CostCenterActivityReportsResponse {
    #[serde(rename = "costCenter")]
    #[serde(default)]
    pub cost_center: CostCenterActivityReportsResponseCostCenter,
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: NaiveDate,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: NaiveDate,
    #[serde(default)]
    pub rows: Vec<CostCenterActivityReportsResponseRowsItem>,
}

impl CostCenterActivityReportsResponse {
    pub fn builder() -> CostCenterActivityReportsResponseBuilder {
        <CostCenterActivityReportsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CostCenterActivityReportsResponseBuilder {
    cost_center: Option<CostCenterActivityReportsResponseCostCenter>,
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
    rows: Option<Vec<CostCenterActivityReportsResponseRowsItem>>,
}

impl CostCenterActivityReportsResponseBuilder {
    pub fn cost_center(mut self, value: CostCenterActivityReportsResponseCostCenter) -> Self {
        self.cost_center = Some(value);
        self
    }

    pub fn from_date(mut self, value: NaiveDate) -> Self {
        self.from_date = Some(value);
        self
    }

    pub fn to_date(mut self, value: NaiveDate) -> Self {
        self.to_date = Some(value);
        self
    }

    pub fn rows(mut self, value: Vec<CostCenterActivityReportsResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CostCenterActivityReportsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`cost_center`](CostCenterActivityReportsResponseBuilder::cost_center)
    /// - [`from_date`](CostCenterActivityReportsResponseBuilder::from_date)
    /// - [`to_date`](CostCenterActivityReportsResponseBuilder::to_date)
    /// - [`rows`](CostCenterActivityReportsResponseBuilder::rows)
    pub fn build(self) -> Result<CostCenterActivityReportsResponse, BuildError> {
        Ok(CostCenterActivityReportsResponse {
            cost_center: self
                .cost_center
                .ok_or_else(|| BuildError::missing_field("cost_center"))?,
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
