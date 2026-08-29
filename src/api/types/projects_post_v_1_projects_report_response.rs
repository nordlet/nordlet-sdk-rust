pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ProjectsReportResponse {
    #[serde(default)]
    pub rows: Vec<PostV1ProjectsReportResponseRowsItem>,
}

impl PostV1ProjectsReportResponse {
    pub fn builder() -> PostV1ProjectsReportResponseBuilder {
        <PostV1ProjectsReportResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProjectsReportResponseBuilder {
    rows: Option<Vec<PostV1ProjectsReportResponseRowsItem>>,
}

impl PostV1ProjectsReportResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1ProjectsReportResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ProjectsReportResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1ProjectsReportResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1ProjectsReportResponse, BuildError> {
        Ok(PostV1ProjectsReportResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
