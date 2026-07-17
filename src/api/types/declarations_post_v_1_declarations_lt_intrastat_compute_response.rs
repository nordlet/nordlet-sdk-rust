pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLtIntrastatComputeResponse {
    pub flow: PostV1DeclarationsLtIntrastatComputeResponseFlow,
    #[serde(rename = "referencePeriod")]
    #[serde(default)]
    pub reference_period: String,
    #[serde(rename = "periodStart")]
    #[serde(default)]
    pub period_start: String,
    #[serde(rename = "periodEnd")]
    #[serde(default)]
    pub period_end: String,
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(rename = "fileId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    #[serde(default)]
    pub rows: Vec<PostV1DeclarationsLtIntrastatComputeResponseRowsItem>,
    #[serde(default)]
    pub totals: PostV1DeclarationsLtIntrastatComputeResponseTotals,
    #[serde(default)]
    pub counts: PostV1DeclarationsLtIntrastatComputeResponseCounts,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
    #[serde(default)]
    pub xml: String,
}

impl PostV1DeclarationsLtIntrastatComputeResponse {
    pub fn builder() -> PostV1DeclarationsLtIntrastatComputeResponseBuilder {
        <PostV1DeclarationsLtIntrastatComputeResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtIntrastatComputeResponseBuilder {
    flow: Option<PostV1DeclarationsLtIntrastatComputeResponseFlow>,
    reference_period: Option<String>,
    period_start: Option<String>,
    period_end: Option<String>,
    file_name: Option<String>,
    file_id: Option<String>,
    rows: Option<Vec<PostV1DeclarationsLtIntrastatComputeResponseRowsItem>>,
    totals: Option<PostV1DeclarationsLtIntrastatComputeResponseTotals>,
    counts: Option<PostV1DeclarationsLtIntrastatComputeResponseCounts>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
    xml: Option<String>,
}

impl PostV1DeclarationsLtIntrastatComputeResponseBuilder {
    pub fn flow(mut self, value: PostV1DeclarationsLtIntrastatComputeResponseFlow) -> Self {
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

    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.file_name = Some(value.into());
        self
    }

    pub fn file_id(mut self, value: impl Into<String>) -> Self {
        self.file_id = Some(value.into());
        self
    }

    pub fn rows(
        mut self,
        value: Vec<PostV1DeclarationsLtIntrastatComputeResponseRowsItem>,
    ) -> Self {
        self.rows = Some(value);
        self
    }

    pub fn totals(mut self, value: PostV1DeclarationsLtIntrastatComputeResponseTotals) -> Self {
        self.totals = Some(value);
        self
    }

    pub fn counts(mut self, value: PostV1DeclarationsLtIntrastatComputeResponseCounts) -> Self {
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

    pub fn xml(mut self, value: impl Into<String>) -> Self {
        self.xml = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtIntrastatComputeResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`flow`](PostV1DeclarationsLtIntrastatComputeResponseBuilder::flow)
    /// - [`reference_period`](PostV1DeclarationsLtIntrastatComputeResponseBuilder::reference_period)
    /// - [`period_start`](PostV1DeclarationsLtIntrastatComputeResponseBuilder::period_start)
    /// - [`period_end`](PostV1DeclarationsLtIntrastatComputeResponseBuilder::period_end)
    /// - [`file_name`](PostV1DeclarationsLtIntrastatComputeResponseBuilder::file_name)
    /// - [`rows`](PostV1DeclarationsLtIntrastatComputeResponseBuilder::rows)
    /// - [`totals`](PostV1DeclarationsLtIntrastatComputeResponseBuilder::totals)
    /// - [`counts`](PostV1DeclarationsLtIntrastatComputeResponseBuilder::counts)
    /// - [`warnings`](PostV1DeclarationsLtIntrastatComputeResponseBuilder::warnings)
    /// - [`notes`](PostV1DeclarationsLtIntrastatComputeResponseBuilder::notes)
    /// - [`xml`](PostV1DeclarationsLtIntrastatComputeResponseBuilder::xml)
    pub fn build(self) -> Result<PostV1DeclarationsLtIntrastatComputeResponse, BuildError> {
        Ok(PostV1DeclarationsLtIntrastatComputeResponse {
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
            file_name: self
                .file_name
                .ok_or_else(|| BuildError::missing_field("file_name"))?,
            file_id: self.file_id,
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
            xml: self.xml.ok_or_else(|| BuildError::missing_field("xml"))?,
        })
    }
}
