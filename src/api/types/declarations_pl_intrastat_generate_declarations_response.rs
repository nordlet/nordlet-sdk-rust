pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PlIntrastatGenerateDeclarationsResponse {
    pub flow: PlIntrastatGenerateDeclarationsResponseFlow,
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
    pub rows: Vec<PlIntrastatGenerateDeclarationsResponseRowsItem>,
    #[serde(default)]
    pub totals: PlIntrastatGenerateDeclarationsResponseTotals,
    #[serde(default)]
    pub counts: PlIntrastatGenerateDeclarationsResponseCounts,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
    #[serde(default)]
    pub source: String,
}

impl PlIntrastatGenerateDeclarationsResponse {
    pub fn builder() -> PlIntrastatGenerateDeclarationsResponseBuilder {
        <PlIntrastatGenerateDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlIntrastatGenerateDeclarationsResponseBuilder {
    flow: Option<PlIntrastatGenerateDeclarationsResponseFlow>,
    reference_period: Option<String>,
    period_start: Option<String>,
    period_end: Option<String>,
    nip: Option<String>,
    company_name: Option<String>,
    detailed_threshold: Option<bool>,
    rows: Option<Vec<PlIntrastatGenerateDeclarationsResponseRowsItem>>,
    totals: Option<PlIntrastatGenerateDeclarationsResponseTotals>,
    counts: Option<PlIntrastatGenerateDeclarationsResponseCounts>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
    source: Option<String>,
}

impl PlIntrastatGenerateDeclarationsResponseBuilder {
    pub fn flow(mut self, value: PlIntrastatGenerateDeclarationsResponseFlow) -> Self {
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

    pub fn rows(mut self, value: Vec<PlIntrastatGenerateDeclarationsResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    pub fn totals(mut self, value: PlIntrastatGenerateDeclarationsResponseTotals) -> Self {
        self.totals = Some(value);
        self
    }

    pub fn counts(mut self, value: PlIntrastatGenerateDeclarationsResponseCounts) -> Self {
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

    /// Consumes the builder and constructs a [`PlIntrastatGenerateDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`flow`](PlIntrastatGenerateDeclarationsResponseBuilder::flow)
    /// - [`reference_period`](PlIntrastatGenerateDeclarationsResponseBuilder::reference_period)
    /// - [`period_start`](PlIntrastatGenerateDeclarationsResponseBuilder::period_start)
    /// - [`period_end`](PlIntrastatGenerateDeclarationsResponseBuilder::period_end)
    /// - [`nip`](PlIntrastatGenerateDeclarationsResponseBuilder::nip)
    /// - [`company_name`](PlIntrastatGenerateDeclarationsResponseBuilder::company_name)
    /// - [`detailed_threshold`](PlIntrastatGenerateDeclarationsResponseBuilder::detailed_threshold)
    /// - [`rows`](PlIntrastatGenerateDeclarationsResponseBuilder::rows)
    /// - [`totals`](PlIntrastatGenerateDeclarationsResponseBuilder::totals)
    /// - [`counts`](PlIntrastatGenerateDeclarationsResponseBuilder::counts)
    /// - [`warnings`](PlIntrastatGenerateDeclarationsResponseBuilder::warnings)
    /// - [`notes`](PlIntrastatGenerateDeclarationsResponseBuilder::notes)
    /// - [`source`](PlIntrastatGenerateDeclarationsResponseBuilder::source)
    pub fn build(self) -> Result<PlIntrastatGenerateDeclarationsResponse, BuildError> {
        Ok(PlIntrastatGenerateDeclarationsResponse {
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
