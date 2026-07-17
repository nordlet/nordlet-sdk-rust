pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1ReportsJobsGetResponse {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "reportType")]
    #[serde(default)]
    pub report_type: String,
    pub params: serde_json::Value,
    #[serde(default)]
    pub formats: Vec<String>,
    pub status: PostV1ReportsJobsGetResponseStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<PostV1ReportsJobsGetResponseOutputsItem>>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(rename = "startedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    #[serde(rename = "finishedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<String>,
}

impl PostV1ReportsJobsGetResponse {
    pub fn builder() -> PostV1ReportsJobsGetResponseBuilder {
        <PostV1ReportsJobsGetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsJobsGetResponseBuilder {
    id: Option<String>,
    report_type: Option<String>,
    params: Option<serde_json::Value>,
    formats: Option<Vec<String>>,
    status: Option<PostV1ReportsJobsGetResponseStatus>,
    error: Option<String>,
    outputs: Option<Vec<PostV1ReportsJobsGetResponseOutputsItem>>,
    created_at: Option<String>,
    started_at: Option<String>,
    finished_at: Option<String>,
}

impl PostV1ReportsJobsGetResponseBuilder {
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

    pub fn status(mut self, value: PostV1ReportsJobsGetResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn outputs(mut self, value: Vec<PostV1ReportsJobsGetResponseOutputsItem>) -> Self {
        self.outputs = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn started_at(mut self, value: impl Into<String>) -> Self {
        self.started_at = Some(value.into());
        self
    }

    pub fn finished_at(mut self, value: impl Into<String>) -> Self {
        self.finished_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsJobsGetResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1ReportsJobsGetResponseBuilder::id)
    /// - [`report_type`](PostV1ReportsJobsGetResponseBuilder::report_type)
    /// - [`params`](PostV1ReportsJobsGetResponseBuilder::params)
    /// - [`formats`](PostV1ReportsJobsGetResponseBuilder::formats)
    /// - [`status`](PostV1ReportsJobsGetResponseBuilder::status)
    /// - [`created_at`](PostV1ReportsJobsGetResponseBuilder::created_at)
    pub fn build(self) -> Result<PostV1ReportsJobsGetResponse, BuildError> {
        Ok(PostV1ReportsJobsGetResponse {
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
