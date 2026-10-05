pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OnlineSalesReportsResponse {
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: NaiveDate,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: NaiveDate,
    #[serde(default)]
    pub rows: Vec<OnlineSalesReportsResponseRowsItem>,
}

impl OnlineSalesReportsResponse {
    pub fn builder() -> OnlineSalesReportsResponseBuilder {
        <OnlineSalesReportsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OnlineSalesReportsResponseBuilder {
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
    rows: Option<Vec<OnlineSalesReportsResponseRowsItem>>,
}

impl OnlineSalesReportsResponseBuilder {
    pub fn from_date(mut self, value: NaiveDate) -> Self {
        self.from_date = Some(value);
        self
    }

    pub fn to_date(mut self, value: NaiveDate) -> Self {
        self.to_date = Some(value);
        self
    }

    pub fn rows(mut self, value: Vec<OnlineSalesReportsResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OnlineSalesReportsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_date`](OnlineSalesReportsResponseBuilder::from_date)
    /// - [`to_date`](OnlineSalesReportsResponseBuilder::to_date)
    /// - [`rows`](OnlineSalesReportsResponseBuilder::rows)
    pub fn build(self) -> Result<OnlineSalesReportsResponse, BuildError> {
        Ok(OnlineSalesReportsResponse {
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
