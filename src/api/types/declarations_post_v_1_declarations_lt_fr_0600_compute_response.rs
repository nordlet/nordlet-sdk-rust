pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLtFr0600ComputeResponse {
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
    pub fields: Vec<PostV1DeclarationsLtFr0600ComputeResponseFieldsItem>,
    #[serde(default)]
    pub breakdown: Vec<PostV1DeclarationsLtFr0600ComputeResponseBreakdownItem>,
    #[serde(default)]
    pub counts: PostV1DeclarationsLtFr0600ComputeResponseCounts,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
}

impl PostV1DeclarationsLtFr0600ComputeResponse {
    pub fn builder() -> PostV1DeclarationsLtFr0600ComputeResponseBuilder {
        <PostV1DeclarationsLtFr0600ComputeResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtFr0600ComputeResponseBuilder {
    period_start: Option<String>,
    period_end: Option<String>,
    deduction_percent: Option<i64>,
    fields: Option<Vec<PostV1DeclarationsLtFr0600ComputeResponseFieldsItem>>,
    breakdown: Option<Vec<PostV1DeclarationsLtFr0600ComputeResponseBreakdownItem>>,
    counts: Option<PostV1DeclarationsLtFr0600ComputeResponseCounts>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
}

impl PostV1DeclarationsLtFr0600ComputeResponseBuilder {
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

    pub fn fields(
        mut self,
        value: Vec<PostV1DeclarationsLtFr0600ComputeResponseFieldsItem>,
    ) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn breakdown(
        mut self,
        value: Vec<PostV1DeclarationsLtFr0600ComputeResponseBreakdownItem>,
    ) -> Self {
        self.breakdown = Some(value);
        self
    }

    pub fn counts(mut self, value: PostV1DeclarationsLtFr0600ComputeResponseCounts) -> Self {
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtFr0600ComputeResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`period_start`](PostV1DeclarationsLtFr0600ComputeResponseBuilder::period_start)
    /// - [`period_end`](PostV1DeclarationsLtFr0600ComputeResponseBuilder::period_end)
    /// - [`deduction_percent`](PostV1DeclarationsLtFr0600ComputeResponseBuilder::deduction_percent)
    /// - [`fields`](PostV1DeclarationsLtFr0600ComputeResponseBuilder::fields)
    /// - [`breakdown`](PostV1DeclarationsLtFr0600ComputeResponseBuilder::breakdown)
    /// - [`counts`](PostV1DeclarationsLtFr0600ComputeResponseBuilder::counts)
    /// - [`warnings`](PostV1DeclarationsLtFr0600ComputeResponseBuilder::warnings)
    /// - [`notes`](PostV1DeclarationsLtFr0600ComputeResponseBuilder::notes)
    pub fn build(self) -> Result<PostV1DeclarationsLtFr0600ComputeResponse, BuildError> {
        Ok(PostV1DeclarationsLtFr0600ComputeResponse {
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
