pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtFr0600ComputeDeclarationsResponse {
    #[serde(rename = "periodStart")]
    #[serde(default)]
    pub period_start: String,
    #[serde(rename = "periodEnd")]
    #[serde(default)]
    pub period_end: String,
    #[serde(rename = "deductionPercent")]
    #[serde(default)]
    pub deduction_percent: i64,
    #[serde(default)]
    pub fields: Vec<LtFr0600ComputeDeclarationsResponseFieldsItem>,
    #[serde(default)]
    pub breakdown: Vec<LtFr0600ComputeDeclarationsResponseBreakdownItem>,
    #[serde(default)]
    pub counts: LtFr0600ComputeDeclarationsResponseCounts,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
}

impl LtFr0600ComputeDeclarationsResponse {
    pub fn builder() -> LtFr0600ComputeDeclarationsResponseBuilder {
        <LtFr0600ComputeDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtFr0600ComputeDeclarationsResponseBuilder {
    period_start: Option<String>,
    period_end: Option<String>,
    deduction_percent: Option<i64>,
    fields: Option<Vec<LtFr0600ComputeDeclarationsResponseFieldsItem>>,
    breakdown: Option<Vec<LtFr0600ComputeDeclarationsResponseBreakdownItem>>,
    counts: Option<LtFr0600ComputeDeclarationsResponseCounts>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
}

impl LtFr0600ComputeDeclarationsResponseBuilder {
    pub fn period_start(mut self, value: impl Into<String>) -> Self {
        self.period_start = Some(value.into());
        self
    }

    pub fn period_end(mut self, value: impl Into<String>) -> Self {
        self.period_end = Some(value.into());
        self
    }

    pub fn deduction_percent(mut self, value: i64) -> Self {
        self.deduction_percent = Some(value);
        self
    }

    pub fn fields(mut self, value: Vec<LtFr0600ComputeDeclarationsResponseFieldsItem>) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn breakdown(
        mut self,
        value: Vec<LtFr0600ComputeDeclarationsResponseBreakdownItem>,
    ) -> Self {
        self.breakdown = Some(value);
        self
    }

    pub fn counts(mut self, value: LtFr0600ComputeDeclarationsResponseCounts) -> Self {
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

    /// Consumes the builder and constructs a [`LtFr0600ComputeDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`period_start`](LtFr0600ComputeDeclarationsResponseBuilder::period_start)
    /// - [`period_end`](LtFr0600ComputeDeclarationsResponseBuilder::period_end)
    /// - [`deduction_percent`](LtFr0600ComputeDeclarationsResponseBuilder::deduction_percent)
    /// - [`fields`](LtFr0600ComputeDeclarationsResponseBuilder::fields)
    /// - [`breakdown`](LtFr0600ComputeDeclarationsResponseBuilder::breakdown)
    /// - [`counts`](LtFr0600ComputeDeclarationsResponseBuilder::counts)
    /// - [`warnings`](LtFr0600ComputeDeclarationsResponseBuilder::warnings)
    /// - [`notes`](LtFr0600ComputeDeclarationsResponseBuilder::notes)
    pub fn build(self) -> Result<LtFr0600ComputeDeclarationsResponse, BuildError> {
        Ok(LtFr0600ComputeDeclarationsResponse {
            period_start: self
                .period_start
                .ok_or_else(|| BuildError::missing_field("period_start"))?,
            period_end: self
                .period_end
                .ok_or_else(|| BuildError::missing_field("period_end"))?,
            deduction_percent: self
                .deduction_percent
                .ok_or_else(|| BuildError::missing_field("deduction_percent"))?,
            fields: self
                .fields
                .ok_or_else(|| BuildError::missing_field("fields"))?,
            breakdown: self
                .breakdown
                .ok_or_else(|| BuildError::missing_field("breakdown"))?,
            counts: self
                .counts
                .ok_or_else(|| BuildError::missing_field("counts"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
            notes: self
                .notes
                .ok_or_else(|| BuildError::missing_field("notes"))?,
        })
    }
}
