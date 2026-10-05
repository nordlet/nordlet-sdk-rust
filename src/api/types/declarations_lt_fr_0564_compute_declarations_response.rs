pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtFr0564ComputeDeclarationsResponse {
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
    pub rows: Vec<LtFr0564ComputeDeclarationsResponseRowsItem>,
    #[serde(default)]
    pub totals: LtFr0564ComputeDeclarationsResponseTotals,
    #[serde(default)]
    pub counts: LtFr0564ComputeDeclarationsResponseCounts,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
    #[serde(default)]
    pub source: String,
}

impl LtFr0564ComputeDeclarationsResponse {
    pub fn builder() -> LtFr0564ComputeDeclarationsResponseBuilder {
        <LtFr0564ComputeDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtFr0564ComputeDeclarationsResponseBuilder {
    year: Option<i64>,
    month: Option<i64>,
    period_start: Option<String>,
    period_end: Option<String>,
    registration_number: Option<String>,
    vat_code: Option<String>,
    company_name: Option<String>,
    rows: Option<Vec<LtFr0564ComputeDeclarationsResponseRowsItem>>,
    totals: Option<LtFr0564ComputeDeclarationsResponseTotals>,
    counts: Option<LtFr0564ComputeDeclarationsResponseCounts>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
    source: Option<String>,
}

impl LtFr0564ComputeDeclarationsResponseBuilder {
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

    pub fn rows(mut self, value: Vec<LtFr0564ComputeDeclarationsResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    pub fn totals(mut self, value: LtFr0564ComputeDeclarationsResponseTotals) -> Self {
        self.totals = Some(value);
        self
    }

    pub fn counts(mut self, value: LtFr0564ComputeDeclarationsResponseCounts) -> Self {
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

    /// Consumes the builder and constructs a [`LtFr0564ComputeDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](LtFr0564ComputeDeclarationsResponseBuilder::year)
    /// - [`month`](LtFr0564ComputeDeclarationsResponseBuilder::month)
    /// - [`period_start`](LtFr0564ComputeDeclarationsResponseBuilder::period_start)
    /// - [`period_end`](LtFr0564ComputeDeclarationsResponseBuilder::period_end)
    /// - [`registration_number`](LtFr0564ComputeDeclarationsResponseBuilder::registration_number)
    /// - [`vat_code`](LtFr0564ComputeDeclarationsResponseBuilder::vat_code)
    /// - [`company_name`](LtFr0564ComputeDeclarationsResponseBuilder::company_name)
    /// - [`rows`](LtFr0564ComputeDeclarationsResponseBuilder::rows)
    /// - [`totals`](LtFr0564ComputeDeclarationsResponseBuilder::totals)
    /// - [`counts`](LtFr0564ComputeDeclarationsResponseBuilder::counts)
    /// - [`warnings`](LtFr0564ComputeDeclarationsResponseBuilder::warnings)
    /// - [`notes`](LtFr0564ComputeDeclarationsResponseBuilder::notes)
    /// - [`source`](LtFr0564ComputeDeclarationsResponseBuilder::source)
    pub fn build(self) -> Result<LtFr0564ComputeDeclarationsResponse, BuildError> {
        Ok(LtFr0564ComputeDeclarationsResponse {
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
