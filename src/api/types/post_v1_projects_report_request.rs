pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ProjectsReportRequest {
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(rename = "dateFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_from: Option<String>,
    #[serde(rename = "dateTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_to: Option<String>,
}

impl PostV1ProjectsReportRequest {
    pub fn builder() -> PostV1ProjectsReportRequestBuilder {
        <PostV1ProjectsReportRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProjectsReportRequestBuilder {
    project_id: Option<String>,
    date_from: Option<String>,
    date_to: Option<String>,
}

impl PostV1ProjectsReportRequestBuilder {
    pub fn project_id(mut self, value: impl Into<String>) -> Self {
        self.project_id = Some(value.into());
        self
    }

    pub fn date_from(mut self, value: impl Into<String>) -> Self {
        self.date_from = Some(value.into());
        self
    }

    pub fn date_to(mut self, value: impl Into<String>) -> Self {
        self.date_to = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ProjectsReportRequest`].
    pub fn build(self) -> Result<PostV1ProjectsReportRequest, BuildError> {
        Ok(PostV1ProjectsReportRequest {
            project_id: self.project_id,
            date_from: self.date_from,
            date_to: self.date_to,
        })
    }
}
