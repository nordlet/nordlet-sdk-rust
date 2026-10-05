pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EuOssComputeDeclarationsResponse {
    #[serde(rename = "periodYear")]
    #[serde(default)]
    pub period_year: i64,
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: NaiveDate,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: NaiveDate,
    #[serde(rename = "memberStateOfIdentification")]
    #[serde(default)]
    pub member_state_of_identification: String,
    #[serde(default)]
    pub rows: Vec<EuOssComputeDeclarationsResponseRowsItem>,
    #[serde(default)]
    pub totals: EuOssComputeDeclarationsResponseTotals,
    #[serde(default)]
    pub corrections: Vec<EuOssComputeDeclarationsResponseCorrectionsItem>,
    #[serde(rename = "correctionsTotal")]
    #[serde(default)]
    pub corrections_total: EuOssComputeDeclarationsResponseCorrectionsTotal,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(rename = "periodQuarter")]
    #[serde(default)]
    pub period_quarter: i64,
}

impl EuOssComputeDeclarationsResponse {
    pub fn builder() -> EuOssComputeDeclarationsResponseBuilder {
        <EuOssComputeDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EuOssComputeDeclarationsResponseBuilder {
    period_year: Option<i64>,
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
    member_state_of_identification: Option<String>,
    rows: Option<Vec<EuOssComputeDeclarationsResponseRowsItem>>,
    totals: Option<EuOssComputeDeclarationsResponseTotals>,
    corrections: Option<Vec<EuOssComputeDeclarationsResponseCorrectionsItem>>,
    corrections_total: Option<EuOssComputeDeclarationsResponseCorrectionsTotal>,
    warnings: Option<Vec<String>>,
    period_quarter: Option<i64>,
}

impl EuOssComputeDeclarationsResponseBuilder {
    pub fn period_year(mut self, value: i64) -> Self {
        self.period_year = Some(value);
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

    pub fn member_state_of_identification(mut self, value: impl Into<String>) -> Self {
        self.member_state_of_identification = Some(value.into());
        self
    }

    pub fn rows(mut self, value: Vec<EuOssComputeDeclarationsResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    pub fn totals(mut self, value: EuOssComputeDeclarationsResponseTotals) -> Self {
        self.totals = Some(value);
        self
    }

    pub fn corrections(
        mut self,
        value: Vec<EuOssComputeDeclarationsResponseCorrectionsItem>,
    ) -> Self {
        self.corrections = Some(value);
        self
    }

    pub fn corrections_total(
        mut self,
        value: EuOssComputeDeclarationsResponseCorrectionsTotal,
    ) -> Self {
        self.corrections_total = Some(value);
        self
    }

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    pub fn period_quarter(mut self, value: i64) -> Self {
        self.period_quarter = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EuOssComputeDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`period_year`](EuOssComputeDeclarationsResponseBuilder::period_year)
    /// - [`from_date`](EuOssComputeDeclarationsResponseBuilder::from_date)
    /// - [`to_date`](EuOssComputeDeclarationsResponseBuilder::to_date)
    /// - [`member_state_of_identification`](EuOssComputeDeclarationsResponseBuilder::member_state_of_identification)
    /// - [`rows`](EuOssComputeDeclarationsResponseBuilder::rows)
    /// - [`totals`](EuOssComputeDeclarationsResponseBuilder::totals)
    /// - [`corrections`](EuOssComputeDeclarationsResponseBuilder::corrections)
    /// - [`corrections_total`](EuOssComputeDeclarationsResponseBuilder::corrections_total)
    /// - [`warnings`](EuOssComputeDeclarationsResponseBuilder::warnings)
    /// - [`period_quarter`](EuOssComputeDeclarationsResponseBuilder::period_quarter)
    pub fn build(self) -> Result<EuOssComputeDeclarationsResponse, BuildError> {
        Ok(EuOssComputeDeclarationsResponse {
            period_year: self
                .period_year
                .ok_or_else(|| BuildError::missing_field("period_year"))?,
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
            member_state_of_identification: self
                .member_state_of_identification
                .ok_or_else(|| BuildError::missing_field("member_state_of_identification"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
            totals: self
                .totals
                .ok_or_else(|| BuildError::missing_field("totals"))?,
            corrections: self
                .corrections
                .ok_or_else(|| BuildError::missing_field("corrections"))?,
            corrections_total: self
                .corrections_total
                .ok_or_else(|| BuildError::missing_field("corrections_total"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
            period_quarter: self
                .period_quarter
                .ok_or_else(|| BuildError::missing_field("period_quarter"))?,
        })
    }
}
