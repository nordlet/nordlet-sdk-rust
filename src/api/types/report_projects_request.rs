pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReportProjectsRequest {
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(rename = "dateFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_from: Option<NaiveDate>,
    #[serde(rename = "dateTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_to: Option<NaiveDate>,
}

impl ReportProjectsRequest {
    pub fn builder() -> ReportProjectsRequestBuilder {
        <ReportProjectsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReportProjectsRequestBuilder {
    project_id: Option<String>,
    date_from: Option<NaiveDate>,
    date_to: Option<NaiveDate>,
}

impl ReportProjectsRequestBuilder {
    pub fn project_id(mut self, value: impl Into<String>) -> Self {
        self.project_id = Some(value.into());
        self
    }

    pub fn date_from(mut self, value: NaiveDate) -> Self {
        self.date_from = Some(value);
        self
    }

    pub fn date_to(mut self, value: NaiveDate) -> Self {
        self.date_to = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReportProjectsRequest`].
    pub fn build(self) -> Result<ReportProjectsRequest, BuildError> {
        Ok(ReportProjectsRequest {
            project_id: self.project_id,
            date_from: self.date_from,
            date_to: self.date_to,
        })
    }
}
