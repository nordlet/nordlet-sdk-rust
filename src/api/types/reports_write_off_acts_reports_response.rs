pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WriteOffActsReportsResponse {
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: NaiveDate,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: NaiveDate,
    #[serde(default)]
    pub rows: Vec<WriteOffActsReportsResponseRowsItem>,
    #[serde(rename = "totalCost")]
    #[serde(default)]
    pub total_cost: String,
}

impl WriteOffActsReportsResponse {
    pub fn builder() -> WriteOffActsReportsResponseBuilder {
        <WriteOffActsReportsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WriteOffActsReportsResponseBuilder {
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
    rows: Option<Vec<WriteOffActsReportsResponseRowsItem>>,
    total_cost: Option<String>,
}

impl WriteOffActsReportsResponseBuilder {
    pub fn from_date(mut self, value: NaiveDate) -> Self {
        self.from_date = Some(value);
        self
    }

    pub fn to_date(mut self, value: NaiveDate) -> Self {
        self.to_date = Some(value);
        self
    }

    pub fn rows(mut self, value: Vec<WriteOffActsReportsResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    pub fn total_cost(mut self, value: impl Into<String>) -> Self {
        self.total_cost = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WriteOffActsReportsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_date`](WriteOffActsReportsResponseBuilder::from_date)
    /// - [`to_date`](WriteOffActsReportsResponseBuilder::to_date)
    /// - [`rows`](WriteOffActsReportsResponseBuilder::rows)
    /// - [`total_cost`](WriteOffActsReportsResponseBuilder::total_cost)
    pub fn build(self) -> Result<WriteOffActsReportsResponse, BuildError> {
        Ok(WriteOffActsReportsResponse {
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
            total_cost: self
                .total_cost
                .ok_or_else(|| BuildError::missing_field("total_cost"))?,
        })
    }
}
