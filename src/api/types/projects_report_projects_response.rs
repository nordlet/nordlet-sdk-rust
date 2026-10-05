pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReportProjectsResponse {
    #[serde(default)]
    pub rows: Vec<ReportProjectsResponseRowsItem>,
}

impl ReportProjectsResponse {
    pub fn builder() -> ReportProjectsResponseBuilder {
        <ReportProjectsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReportProjectsResponseBuilder {
    rows: Option<Vec<ReportProjectsResponseRowsItem>>,
}

impl ReportProjectsResponseBuilder {
    pub fn rows(mut self, value: Vec<ReportProjectsResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReportProjectsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](ReportProjectsResponseBuilder::rows)
    pub fn build(self) -> Result<ReportProjectsResponse, BuildError> {
        Ok(ReportProjectsResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
