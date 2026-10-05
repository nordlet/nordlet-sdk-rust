pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VatDetailReportsResponse {
    #[serde(default)]
    pub side: String,
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: NaiveDate,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: NaiveDate,
    #[serde(default)]
    pub rows: Vec<VatDetailReportsResponseRowsItem>,
    #[serde(default)]
    pub totals: VatDetailReportsResponseTotals,
}

impl VatDetailReportsResponse {
    pub fn builder() -> VatDetailReportsResponseBuilder {
        <VatDetailReportsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VatDetailReportsResponseBuilder {
    side: Option<String>,
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
    rows: Option<Vec<VatDetailReportsResponseRowsItem>>,
    totals: Option<VatDetailReportsResponseTotals>,
}

impl VatDetailReportsResponseBuilder {
    pub fn side(mut self, value: impl Into<String>) -> Self {
        self.side = Some(value.into());
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

    pub fn rows(mut self, value: Vec<VatDetailReportsResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    pub fn totals(mut self, value: VatDetailReportsResponseTotals) -> Self {
        self.totals = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VatDetailReportsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`side`](VatDetailReportsResponseBuilder::side)
    /// - [`from_date`](VatDetailReportsResponseBuilder::from_date)
    /// - [`to_date`](VatDetailReportsResponseBuilder::to_date)
    /// - [`rows`](VatDetailReportsResponseBuilder::rows)
    /// - [`totals`](VatDetailReportsResponseBuilder::totals)
    pub fn build(self) -> Result<VatDetailReportsResponse, BuildError> {
        Ok(VatDetailReportsResponse {
            side: self.side.ok_or_else(|| BuildError::missing_field("side"))?,
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
