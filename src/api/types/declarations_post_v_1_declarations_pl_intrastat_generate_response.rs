pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsPlIntrastatGenerateResponse {
    pub flow: PostV1DeclarationsPlIntrastatGenerateResponseFlow,
    #[serde(rename = "referencePeriod")]
    #[serde(default)]
    pub reference_period: String,
    #[serde(rename = "periodStart")]
    #[serde(default)]
    pub period_start: String,
    #[serde(rename = "periodEnd")]
    #[serde(default)]
    pub period_end: String,
    #[serde(default)]
    pub nip: String,
    #[serde(rename = "companyName")]
    #[serde(default)]
    pub company_name: String,
    #[serde(rename = "detailedThreshold")]
    #[serde(default)]
    pub detailed_threshold: bool,
    #[serde(default)]
    pub rows: Vec<PostV1DeclarationsPlIntrastatGenerateResponseRowsItem>,
    #[serde(default)]
    pub totals: PostV1DeclarationsPlIntrastatGenerateResponseTotals,
    #[serde(default)]
    pub counts: PostV1DeclarationsPlIntrastatGenerateResponseCounts,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
    #[serde(default)]
    pub source: String,
}

impl PostV1DeclarationsPlIntrastatGenerateResponse {
    pub fn builder() -> PostV1DeclarationsPlIntrastatGenerateResponseBuilder {
        <PostV1DeclarationsPlIntrastatGenerateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsPlIntrastatGenerateResponseBuilder {
    flow: Option<PostV1DeclarationsPlIntrastatGenerateResponseFlow>,
    reference_period: Option<String>,
    period_start: Option<String>,
    period_end: Option<String>,
    nip: Option<String>,
    company_name: Option<String>,
    detailed_threshold: Option<bool>,
    rows: Option<Vec<PostV1DeclarationsPlIntrastatGenerateResponseRowsItem>>,
    totals: Option<PostV1DeclarationsPlIntrastatGenerateResponseTotals>,
    counts: Option<PostV1DeclarationsPlIntrastatGenerateResponseCounts>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
    source: Option<String>,
}

impl PostV1DeclarationsPlIntrastatGenerateResponseBuilder {
    pub fn flow(mut self, value: PostV1DeclarationsPlIntrastatGenerateResponseFlow) -> Self {
        self.flow = Some(value);
        self
    }

    pub fn reference_period(mut self, value: impl Into<String>) -> Self {
        self.reference_period = Some(value.into());
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

    pub fn nip(mut self, value: impl Into<String>) -> Self {
        self.nip = Some(value.into());
        self
    }

    pub fn company_name(mut self, value: impl Into<String>) -> Self {
        self.company_name = Some(value.into());
        self
    }

    pub fn detailed_threshold(mut self, value: bool) -> Self {
        self.detailed_threshold = Some(value);
        self
    }

    pub fn rows(
        mut self,
        value: Vec<PostV1DeclarationsPlIntrastatGenerateResponseRowsItem>,
    ) -> Self {
        self.rows = Some(value);
        self
    }

    pub fn totals(mut self, value: PostV1DeclarationsPlIntrastatGenerateResponseTotals) -> Self {
        self.totals = Some(value);
        self
    }

    pub fn counts(mut self, value: PostV1DeclarationsPlIntrastatGenerateResponseCounts) -> Self {
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsPlIntrastatGenerateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`flow`](PostV1DeclarationsPlIntrastatGenerateResponseBuilder::flow)
    /// - [`reference_period`](PostV1DeclarationsPlIntrastatGenerateResponseBuilder::reference_period)
    /// - [`period_start`](PostV1DeclarationsPlIntrastatGenerateResponseBuilder::period_start)
    /// - [`period_end`](PostV1DeclarationsPlIntrastatGenerateResponseBuilder::period_end)
    /// - [`nip`](PostV1DeclarationsPlIntrastatGenerateResponseBuilder::nip)
    /// - [`company_name`](PostV1DeclarationsPlIntrastatGenerateResponseBuilder::company_name)
    /// - [`detailed_threshold`](PostV1DeclarationsPlIntrastatGenerateResponseBuilder::detailed_threshold)
    /// - [`rows`](PostV1DeclarationsPlIntrastatGenerateResponseBuilder::rows)
    /// - [`totals`](PostV1DeclarationsPlIntrastatGenerateResponseBuilder::totals)
    /// - [`counts`](PostV1DeclarationsPlIntrastatGenerateResponseBuilder::counts)
    /// - [`warnings`](PostV1DeclarationsPlIntrastatGenerateResponseBuilder::warnings)
    /// - [`notes`](PostV1DeclarationsPlIntrastatGenerateResponseBuilder::notes)
    /// - [`source`](PostV1DeclarationsPlIntrastatGenerateResponseBuilder::source)
    pub fn build(self) -> Result<PostV1DeclarationsPlIntrastatGenerateResponse, BuildError> {
        Ok(PostV1DeclarationsPlIntrastatGenerateResponse {
            flow: self.flow.ok_or_else(|| BuildError::missing_field("flow"))?,
            reference_period: self
                .reference_period
                .ok_or_else(|| BuildError::missing_field("reference_period"))?,
            period_start: self
                .period_start
                .ok_or_else(|| BuildError::missing_field("period_start"))?,
            period_end: self
                .period_end
                .ok_or_else(|| BuildError::missing_field("period_end"))?,
            nip: self.nip.ok_or_else(|| BuildError::missing_field("nip"))?,
            company_name: self
                .company_name
                .ok_or_else(|| BuildError::missing_field("company_name"))?,
            detailed_threshold: self
                .detailed_threshold
                .ok_or_else(|| BuildError::missing_field("detailed_threshold"))?,
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
