pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsEuOssComputeResponse {
    #[serde(rename = "periodYear")]
    #[serde(default)]
    pub period_year: i64,
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: String,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: String,
    #[serde(rename = "memberStateOfIdentification")]
    #[serde(default)]
    pub member_state_of_identification: String,
    #[serde(default)]
    pub rows: Vec<PostV1DeclarationsEuOssComputeResponseRowsItem>,
    #[serde(default)]
    pub totals: PostV1DeclarationsEuOssComputeResponseTotals,
    #[serde(default)]
    pub corrections: Vec<PostV1DeclarationsEuOssComputeResponseCorrectionsItem>,
    #[serde(rename = "correctionsTotal")]
    #[serde(default)]
    pub corrections_total: PostV1DeclarationsEuOssComputeResponseCorrectionsTotal,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(rename = "periodQuarter")]
    #[serde(default)]
    pub period_quarter: i64,
}

impl PostV1DeclarationsEuOssComputeResponse {
    pub fn builder() -> PostV1DeclarationsEuOssComputeResponseBuilder {
        <PostV1DeclarationsEuOssComputeResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsEuOssComputeResponseBuilder {
    period_year: Option<i64>,
    from_date: Option<String>,
    to_date: Option<String>,
    member_state_of_identification: Option<String>,
    rows: Option<Vec<PostV1DeclarationsEuOssComputeResponseRowsItem>>,
    totals: Option<PostV1DeclarationsEuOssComputeResponseTotals>,
    corrections: Option<Vec<PostV1DeclarationsEuOssComputeResponseCorrectionsItem>>,
    corrections_total: Option<PostV1DeclarationsEuOssComputeResponseCorrectionsTotal>,
    warnings: Option<Vec<String>>,
    period_quarter: Option<i64>,
}

impl PostV1DeclarationsEuOssComputeResponseBuilder {
    pub fn period_year(mut self, value: i64) -> Self {
        self.period_year = Some(value);
        self
    }

    pub fn from_date(mut self, value: impl Into<String>) -> Self {
        self.from_date = Some(value.into());
        self
    }

    pub fn to_date(mut self, value: impl Into<String>) -> Self {
        self.to_date = Some(value.into());
        self
    }

    pub fn member_state_of_identification(mut self, value: impl Into<String>) -> Self {
        self.member_state_of_identification = Some(value.into());
        self
    }

    pub fn rows(mut self, value: Vec<PostV1DeclarationsEuOssComputeResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    pub fn totals(mut self, value: PostV1DeclarationsEuOssComputeResponseTotals) -> Self {
        self.totals = Some(value);
        self
    }

    pub fn corrections(
        mut self,
        value: Vec<PostV1DeclarationsEuOssComputeResponseCorrectionsItem>,
    ) -> Self {
        self.corrections = Some(value);
        self
    }

    pub fn corrections_total(
        mut self,
        value: PostV1DeclarationsEuOssComputeResponseCorrectionsTotal,
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsEuOssComputeResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`period_year`](PostV1DeclarationsEuOssComputeResponseBuilder::period_year)
    /// - [`from_date`](PostV1DeclarationsEuOssComputeResponseBuilder::from_date)
    /// - [`to_date`](PostV1DeclarationsEuOssComputeResponseBuilder::to_date)
    /// - [`member_state_of_identification`](PostV1DeclarationsEuOssComputeResponseBuilder::member_state_of_identification)
    /// - [`rows`](PostV1DeclarationsEuOssComputeResponseBuilder::rows)
    /// - [`totals`](PostV1DeclarationsEuOssComputeResponseBuilder::totals)
    /// - [`corrections`](PostV1DeclarationsEuOssComputeResponseBuilder::corrections)
    /// - [`corrections_total`](PostV1DeclarationsEuOssComputeResponseBuilder::corrections_total)
    /// - [`warnings`](PostV1DeclarationsEuOssComputeResponseBuilder::warnings)
    /// - [`period_quarter`](PostV1DeclarationsEuOssComputeResponseBuilder::period_quarter)
    pub fn build(self) -> Result<PostV1DeclarationsEuOssComputeResponse, BuildError> {
        Ok(PostV1DeclarationsEuOssComputeResponse {
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
