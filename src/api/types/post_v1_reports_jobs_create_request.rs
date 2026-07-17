pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostV1ReportsJobsCreateRequest {
    #[serde(rename = "reportType")]
    #[serde(default)]
    pub report_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formats: Option<Vec<PostV1ReportsJobsCreateRequestFormatsItem>>,
}

impl PostV1ReportsJobsCreateRequest {
    pub fn builder() -> PostV1ReportsJobsCreateRequestBuilder {
        <PostV1ReportsJobsCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsJobsCreateRequestBuilder {
    report_type: Option<String>,
    params: Option<HashMap<String, serde_json::Value>>,
    formats: Option<Vec<PostV1ReportsJobsCreateRequestFormatsItem>>,
}

impl PostV1ReportsJobsCreateRequestBuilder {
    pub fn report_type(mut self, value: impl Into<String>) -> Self {
        self.report_type = Some(value.into());
        self
    }

    pub fn params(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.params = Some(value);
        self
    }

    pub fn formats(mut self, value: Vec<PostV1ReportsJobsCreateRequestFormatsItem>) -> Self {
        self.formats = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsJobsCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`report_type`](PostV1ReportsJobsCreateRequestBuilder::report_type)
    pub fn build(self) -> Result<PostV1ReportsJobsCreateRequest, BuildError> {
        Ok(PostV1ReportsJobsCreateRequest {
            report_type: self
                .report_type
                .ok_or_else(|| BuildError::missing_field("report_type"))?,
            params: self.params,
            formats: self.formats,
        })
    }
}
