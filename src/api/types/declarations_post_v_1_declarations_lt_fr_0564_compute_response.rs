pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLtFr0564ComputeResponse {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
    #[serde(rename = "periodStart")]
    #[serde(default)]
    pub period_start: String,
    #[serde(rename = "periodEnd")]
    #[serde(default)]
    pub period_end: String,
    #[serde(rename = "registrationNumber")]
    #[serde(default)]
    pub registration_number: String,
    #[serde(rename = "vatCode")]
    #[serde(default)]
    pub vat_code: String,
    #[serde(rename = "companyName")]
    #[serde(default)]
    pub company_name: String,
    #[serde(default)]
    pub rows: Vec<PostV1DeclarationsLtFr0564ComputeResponseRowsItem>,
    #[serde(default)]
    pub totals: PostV1DeclarationsLtFr0564ComputeResponseTotals,
    #[serde(default)]
    pub counts: PostV1DeclarationsLtFr0564ComputeResponseCounts,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
    #[serde(default)]
    pub source: String,
}

impl PostV1DeclarationsLtFr0564ComputeResponse {
    pub fn builder() -> PostV1DeclarationsLtFr0564ComputeResponseBuilder {
        <PostV1DeclarationsLtFr0564ComputeResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtFr0564ComputeResponseBuilder {
    year: Option<i64>,
    month: Option<i64>,
    period_start: Option<String>,
    period_end: Option<String>,
    registration_number: Option<String>,
    vat_code: Option<String>,
    company_name: Option<String>,
    rows: Option<Vec<PostV1DeclarationsLtFr0564ComputeResponseRowsItem>>,
    totals: Option<PostV1DeclarationsLtFr0564ComputeResponseTotals>,
    counts: Option<PostV1DeclarationsLtFr0564ComputeResponseCounts>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
    source: Option<String>,
}

impl PostV1DeclarationsLtFr0564ComputeResponseBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    pub fn period_start(mut self, value: impl Into<String>) -> Self {
        self.period_start = Some(value.into());
        self
    }

    pub fn period_end(mut self, value: impl Into<String>) -> Self {
        self.period_end = Some(value.into());
        self
    }

    pub fn registration_number(mut self, value: impl Into<String>) -> Self {
        self.registration_number = Some(value.into());
        self
    }

    pub fn vat_code(mut self, value: impl Into<String>) -> Self {
        self.vat_code = Some(value.into());
        self
    }

    pub fn company_name(mut self, value: impl Into<String>) -> Self {
        self.company_name = Some(value.into());
        self
    }

    pub fn rows(mut self, value: Vec<PostV1DeclarationsLtFr0564ComputeResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    pub fn totals(mut self, value: PostV1DeclarationsLtFr0564ComputeResponseTotals) -> Self {
        self.totals = Some(value);
        self
    }

    pub fn counts(mut self, value: PostV1DeclarationsLtFr0564ComputeResponseCounts) -> Self {
        self.counts = Some(value);
        self
    }

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    pub fn notes(mut self, value: Vec<String>) -> Self {
        self.notes = Some(value);
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtFr0564ComputeResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsLtFr0564ComputeResponseBuilder::year)
    /// - [`month`](PostV1DeclarationsLtFr0564ComputeResponseBuilder::month)
    /// - [`period_start`](PostV1DeclarationsLtFr0564ComputeResponseBuilder::period_start)
    /// - [`period_end`](PostV1DeclarationsLtFr0564ComputeResponseBuilder::period_end)
    /// - [`registration_number`](PostV1DeclarationsLtFr0564ComputeResponseBuilder::registration_number)
    /// - [`vat_code`](PostV1DeclarationsLtFr0564ComputeResponseBuilder::vat_code)
    /// - [`company_name`](PostV1DeclarationsLtFr0564ComputeResponseBuilder::company_name)
    /// - [`rows`](PostV1DeclarationsLtFr0564ComputeResponseBuilder::rows)
    /// - [`totals`](PostV1DeclarationsLtFr0564ComputeResponseBuilder::totals)
    /// - [`counts`](PostV1DeclarationsLtFr0564ComputeResponseBuilder::counts)
    /// - [`warnings`](PostV1DeclarationsLtFr0564ComputeResponseBuilder::warnings)
    /// - [`notes`](PostV1DeclarationsLtFr0564ComputeResponseBuilder::notes)
    /// - [`source`](PostV1DeclarationsLtFr0564ComputeResponseBuilder::source)
    pub fn build(self) -> Result<PostV1DeclarationsLtFr0564ComputeResponse, BuildError> {
        Ok(PostV1DeclarationsLtFr0564ComputeResponse {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
            period_start: self
                .period_start
                .ok_or_else(|| BuildError::missing_field("period_start"))?,
            period_end: self
                .period_end
                .ok_or_else(|| BuildError::missing_field("period_end"))?,
            registration_number: self
                .registration_number
                .ok_or_else(|| BuildError::missing_field("registration_number"))?,
            vat_code: self
                .vat_code
                .ok_or_else(|| BuildError::missing_field("vat_code"))?,
            company_name: self
                .company_name
                .ok_or_else(|| BuildError::missing_field("company_name"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
            totals: self
                .totals
                .ok_or_else(|| BuildError::missing_field("totals"))?,
            counts: self
                .counts
                .ok_or_else(|| BuildError::missing_field("counts"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
            notes: self
                .notes
                .ok_or_else(|| BuildError::missing_field("notes"))?,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
        })
    }
}
