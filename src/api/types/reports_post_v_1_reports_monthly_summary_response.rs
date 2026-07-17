pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsMonthlySummaryResponse {
    #[serde(default)]
    pub rows: Vec<PostV1ReportsMonthlySummaryResponseRowsItem>,
}

impl PostV1ReportsMonthlySummaryResponse {
    pub fn builder() -> PostV1ReportsMonthlySummaryResponseBuilder {
        <PostV1ReportsMonthlySummaryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsMonthlySummaryResponseBuilder {
    rows: Option<Vec<PostV1ReportsMonthlySummaryResponseRowsItem>>,
}

impl PostV1ReportsMonthlySummaryResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1ReportsMonthlySummaryResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsMonthlySummaryResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1ReportsMonthlySummaryResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1ReportsMonthlySummaryResponse, BuildError> {
        Ok(PostV1ReportsMonthlySummaryResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
