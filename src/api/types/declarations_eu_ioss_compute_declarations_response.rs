pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EuIossComputeDeclarationsResponse {
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
    pub rows: Vec<EuIossComputeDeclarationsResponseRowsItem>,
    #[serde(default)]
    pub totals: EuIossComputeDeclarationsResponseTotals,
    #[serde(default)]
    pub corrections: Vec<EuIossComputeDeclarationsResponseCorrectionsItem>,
    #[serde(rename = "correctionsTotal")]
    #[serde(default)]
    pub corrections_total: EuIossComputeDeclarationsResponseCorrectionsTotal,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(rename = "periodMonth")]
    #[serde(default)]
    pub period_month: i64,
}

impl EuIossComputeDeclarationsResponse {
    pub fn builder() -> EuIossComputeDeclarationsResponseBuilder {
        <EuIossComputeDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EuIossComputeDeclarationsResponseBuilder {
    period_year: Option<i64>,
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
    member_state_of_identification: Option<String>,
    rows: Option<Vec<EuIossComputeDeclarationsResponseRowsItem>>,
    totals: Option<EuIossComputeDeclarationsResponseTotals>,
    corrections: Option<Vec<EuIossComputeDeclarationsResponseCorrectionsItem>>,
    corrections_total: Option<EuIossComputeDeclarationsResponseCorrectionsTotal>,
    warnings: Option<Vec<String>>,
    period_month: Option<i64>,
}

impl EuIossComputeDeclarationsResponseBuilder {
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

    pub fn rows(mut self, value: Vec<EuIossComputeDeclarationsResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    pub fn totals(mut self, value: EuIossComputeDeclarationsResponseTotals) -> Self {
        self.totals = Some(value);
        self
    }

    pub fn corrections(
        mut self,
        value: Vec<EuIossComputeDeclarationsResponseCorrectionsItem>,
    ) -> Self {
        self.corrections = Some(value);
        self
    }

    pub fn corrections_total(
        mut self,
        value: EuIossComputeDeclarationsResponseCorrectionsTotal,
    ) -> Self {
        self.corrections_total = Some(value);
        self
    }

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    pub fn period_month(mut self, value: i64) -> Self {
        self.period_month = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EuIossComputeDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`period_year`](EuIossComputeDeclarationsResponseBuilder::period_year)
    /// - [`from_date`](EuIossComputeDeclarationsResponseBuilder::from_date)
    /// - [`to_date`](EuIossComputeDeclarationsResponseBuilder::to_date)
    /// - [`member_state_of_identification`](EuIossComputeDeclarationsResponseBuilder::member_state_of_identification)
    /// - [`rows`](EuIossComputeDeclarationsResponseBuilder::rows)
    /// - [`totals`](EuIossComputeDeclarationsResponseBuilder::totals)
    /// - [`corrections`](EuIossComputeDeclarationsResponseBuilder::corrections)
    /// - [`corrections_total`](EuIossComputeDeclarationsResponseBuilder::corrections_total)
    /// - [`warnings`](EuIossComputeDeclarationsResponseBuilder::warnings)
    /// - [`period_month`](EuIossComputeDeclarationsResponseBuilder::period_month)
    pub fn build(self) -> Result<EuIossComputeDeclarationsResponse, BuildError> {
        Ok(EuIossComputeDeclarationsResponse {
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
            period_month: self
                .period_month
                .ok_or_else(|| BuildError::missing_field("period_month"))?,
        })
    }
}
