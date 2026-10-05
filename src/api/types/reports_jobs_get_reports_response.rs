pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JobsGetReportsResponse {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "reportType")]
    #[serde(default)]
    pub report_type: String,
    pub params: serde_json::Value,
    #[serde(default)]
    pub formats: Vec<String>,
    pub status: JobsGetReportsResponseStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<JobsGetReportsResponseOutputsItem>>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(rename = "startedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub started_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "finishedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub finished_at: Option<DateTime<FixedOffset>>,
}

impl JobsGetReportsResponse {
    pub fn builder() -> JobsGetReportsResponseBuilder {
        <JobsGetReportsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct JobsGetReportsResponseBuilder {
    id: Option<String>,
    report_type: Option<String>,
    params: Option<serde_json::Value>,
    formats: Option<Vec<String>>,
    status: Option<JobsGetReportsResponseStatus>,
    error: Option<String>,
    outputs: Option<Vec<JobsGetReportsResponseOutputsItem>>,
    created_at: Option<DateTime<FixedOffset>>,
    started_at: Option<DateTime<FixedOffset>>,
    finished_at: Option<DateTime<FixedOffset>>,
}

impl JobsGetReportsResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn report_type(mut self, value: impl Into<String>) -> Self {
        self.report_type = Some(value.into());
        self
    }

    pub fn params(mut self, value: serde_json::Value) -> Self {
        self.params = Some(value);
        self
    }

    pub fn formats(mut self, value: Vec<String>) -> Self {
        self.formats = Some(value);
        self
    }

    pub fn status(mut self, value: JobsGetReportsResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn outputs(mut self, value: Vec<JobsGetReportsResponseOutputsItem>) -> Self {
        self.outputs = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn started_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.started_at = Some(value);
        self
    }

    pub fn finished_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.finished_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`JobsGetReportsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](JobsGetReportsResponseBuilder::id)
    /// - [`report_type`](JobsGetReportsResponseBuilder::report_type)
    /// - [`params`](JobsGetReportsResponseBuilder::params)
    /// - [`formats`](JobsGetReportsResponseBuilder::formats)
    /// - [`status`](JobsGetReportsResponseBuilder::status)
    /// - [`created_at`](JobsGetReportsResponseBuilder::created_at)
    pub fn build(self) -> Result<JobsGetReportsResponse, BuildError> {
        Ok(JobsGetReportsResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            report_type: self
                .report_type
                .ok_or_else(|| BuildError::missing_field("report_type"))?,
            params: self
                .params
                .ok_or_else(|| BuildError::missing_field("params"))?,
            formats: self
                .formats
                .ok_or_else(|| BuildError::missing_field("formats"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            error: self.error,
            outputs: self.outputs,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            started_at: self.started_at,
            finished_at: self.finished_at,
        })
    }
}
