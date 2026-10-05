pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct JobsCreateReportsRequest {
    #[serde(rename = "reportType")]
    #[serde(default)]
    pub report_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formats: Option<Vec<JobsCreateReportsRequestFormatsItem>>,
}

impl JobsCreateReportsRequest {
    pub fn builder() -> JobsCreateReportsRequestBuilder {
        <JobsCreateReportsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct JobsCreateReportsRequestBuilder {
    report_type: Option<String>,
    params: Option<HashMap<String, serde_json::Value>>,
    formats: Option<Vec<JobsCreateReportsRequestFormatsItem>>,
}

impl JobsCreateReportsRequestBuilder {
    pub fn report_type(mut self, value: impl Into<String>) -> Self {
        self.report_type = Some(value.into());
        self
    }

    pub fn params(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.params = Some(value);
        self
    }

    pub fn formats(mut self, value: Vec<JobsCreateReportsRequestFormatsItem>) -> Self {
        self.formats = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`JobsCreateReportsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`report_type`](JobsCreateReportsRequestBuilder::report_type)
    pub fn build(self) -> Result<JobsCreateReportsRequest, BuildError> {
        Ok(JobsCreateReportsRequest {
            report_type: self
                .report_type
                .ok_or_else(|| BuildError::missing_field("report_type"))?,
            params: self.params,
            formats: self.formats,
        })
    }
}
