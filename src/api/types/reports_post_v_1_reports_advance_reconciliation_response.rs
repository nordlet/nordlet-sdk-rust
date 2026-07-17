pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsAdvanceReconciliationResponse {
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: String,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: String,
    #[serde(default)]
    pub rows: Vec<PostV1ReportsAdvanceReconciliationResponseRowsItem>,
}

impl PostV1ReportsAdvanceReconciliationResponse {
    pub fn builder() -> PostV1ReportsAdvanceReconciliationResponseBuilder {
        <PostV1ReportsAdvanceReconciliationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsAdvanceReconciliationResponseBuilder {
    from_date: Option<String>,
    to_date: Option<String>,
    rows: Option<Vec<PostV1ReportsAdvanceReconciliationResponseRowsItem>>,
}

impl PostV1ReportsAdvanceReconciliationResponseBuilder {
    pub fn from_date(mut self, value: impl Into<String>) -> Self {
        self.from_date = Some(value.into());
        self
    }

    pub fn to_date(mut self, value: impl Into<String>) -> Self {
        self.to_date = Some(value.into());
        self
    }

    pub fn rows(mut self, value: Vec<PostV1ReportsAdvanceReconciliationResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsAdvanceReconciliationResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_date`](PostV1ReportsAdvanceReconciliationResponseBuilder::from_date)
    /// - [`to_date`](PostV1ReportsAdvanceReconciliationResponseBuilder::to_date)
    /// - [`rows`](PostV1ReportsAdvanceReconciliationResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1ReportsAdvanceReconciliationResponse, BuildError> {
        Ok(PostV1ReportsAdvanceReconciliationResponse {
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
