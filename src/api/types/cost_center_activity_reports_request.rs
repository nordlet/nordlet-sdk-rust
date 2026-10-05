pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CostCenterActivityReportsRequest {
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: NaiveDate,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: NaiveDate,
    #[serde(rename = "costCenterId")]
    #[serde(default)]
    pub cost_center_id: String,
}

impl CostCenterActivityReportsRequest {
    pub fn builder() -> CostCenterActivityReportsRequestBuilder {
        <CostCenterActivityReportsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CostCenterActivityReportsRequestBuilder {
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
    cost_center_id: Option<String>,
}

impl CostCenterActivityReportsRequestBuilder {
    pub fn from_date(mut self, value: NaiveDate) -> Self {
        self.from_date = Some(value);
        self
    }

    pub fn to_date(mut self, value: NaiveDate) -> Self {
        self.to_date = Some(value);
        self
    }

    pub fn cost_center_id(mut self, value: impl Into<String>) -> Self {
        self.cost_center_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CostCenterActivityReportsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_date`](CostCenterActivityReportsRequestBuilder::from_date)
    /// - [`to_date`](CostCenterActivityReportsRequestBuilder::to_date)
    /// - [`cost_center_id`](CostCenterActivityReportsRequestBuilder::cost_center_id)
    pub fn build(self) -> Result<CostCenterActivityReportsRequest, BuildError> {
        Ok(CostCenterActivityReportsRequest {
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
            cost_center_id: self
                .cost_center_id
                .ok_or_else(|| BuildError::missing_field("cost_center_id"))?,
        })
    }
}
