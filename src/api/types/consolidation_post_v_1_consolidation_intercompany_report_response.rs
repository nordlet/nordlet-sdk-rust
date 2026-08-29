pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ConsolidationIntercompanyReportResponse {
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: String,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: String,
    #[serde(default)]
    pub directions: Vec<PostV1ConsolidationIntercompanyReportResponseDirectionsItem>,
}

impl PostV1ConsolidationIntercompanyReportResponse {
    pub fn builder() -> PostV1ConsolidationIntercompanyReportResponseBuilder {
        <PostV1ConsolidationIntercompanyReportResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ConsolidationIntercompanyReportResponseBuilder {
    from_date: Option<String>,
    to_date: Option<String>,
    directions: Option<Vec<PostV1ConsolidationIntercompanyReportResponseDirectionsItem>>,
}

impl PostV1ConsolidationIntercompanyReportResponseBuilder {
    pub fn from_date(mut self, value: impl Into<String>) -> Self {
        self.from_date = Some(value.into());
        self
    }

    pub fn to_date(mut self, value: impl Into<String>) -> Self {
        self.to_date = Some(value.into());
        self
    }

    pub fn directions(
        mut self,
        value: Vec<PostV1ConsolidationIntercompanyReportResponseDirectionsItem>,
    ) -> Self {
        self.directions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ConsolidationIntercompanyReportResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_date`](PostV1ConsolidationIntercompanyReportResponseBuilder::from_date)
    /// - [`to_date`](PostV1ConsolidationIntercompanyReportResponseBuilder::to_date)
    /// - [`directions`](PostV1ConsolidationIntercompanyReportResponseBuilder::directions)
    pub fn build(self) -> Result<PostV1ConsolidationIntercompanyReportResponse, BuildError> {
        Ok(PostV1ConsolidationIntercompanyReportResponse {
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
            directions: self
                .directions
                .ok_or_else(|| BuildError::missing_field("directions"))?,
        })
    }
}
