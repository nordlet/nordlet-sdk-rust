pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CostCenterItemsReportsRequest {
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: NaiveDate,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: NaiveDate,
    #[serde(rename = "costCenterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_center_id: Option<String>,
}

impl CostCenterItemsReportsRequest {
    pub fn builder() -> CostCenterItemsReportsRequestBuilder {
        <CostCenterItemsReportsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CostCenterItemsReportsRequestBuilder {
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
    cost_center_id: Option<String>,
}

impl CostCenterItemsReportsRequestBuilder {
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

    /// Consumes the builder and constructs a [`CostCenterItemsReportsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_date`](CostCenterItemsReportsRequestBuilder::from_date)
    /// - [`to_date`](CostCenterItemsReportsRequestBuilder::to_date)
    pub fn build(self) -> Result<CostCenterItemsReportsRequest, BuildError> {
        Ok(CostCenterItemsReportsRequest {
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
            cost_center_id: self.cost_center_id,
        })
    }
}
