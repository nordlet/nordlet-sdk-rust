pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OssReportsResponse {
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: NaiveDate,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: NaiveDate,
    #[serde(default)]
    pub rows: Vec<OssReportsResponseRowsItem>,
    #[serde(default)]
    pub totals: OssReportsResponseTotals,
}

impl OssReportsResponse {
    pub fn builder() -> OssReportsResponseBuilder {
        <OssReportsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OssReportsResponseBuilder {
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
    rows: Option<Vec<OssReportsResponseRowsItem>>,
    totals: Option<OssReportsResponseTotals>,
}

impl OssReportsResponseBuilder {
    pub fn from_date(mut self, value: NaiveDate) -> Self {
        self.from_date = Some(value);
        self
    }

    pub fn to_date(mut self, value: NaiveDate) -> Self {
        self.to_date = Some(value);
        self
    }

    pub fn rows(mut self, value: Vec<OssReportsResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    pub fn totals(mut self, value: OssReportsResponseTotals) -> Self {
        self.totals = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OssReportsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_date`](OssReportsResponseBuilder::from_date)
    /// - [`to_date`](OssReportsResponseBuilder::to_date)
    /// - [`rows`](OssReportsResponseBuilder::rows)
    /// - [`totals`](OssReportsResponseBuilder::totals)
    pub fn build(self) -> Result<OssReportsResponse, BuildError> {
        Ok(OssReportsResponse {
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
            totals: self
                .totals
                .ok_or_else(|| BuildError::missing_field("totals"))?,
        })
    }
}
