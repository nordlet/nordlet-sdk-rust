pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsCostCenterItemsResponse {
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: String,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: String,
    #[serde(default)]
    pub rows: Vec<PostV1ReportsCostCenterItemsResponseRowsItem>,
}

impl PostV1ReportsCostCenterItemsResponse {
    pub fn builder() -> PostV1ReportsCostCenterItemsResponseBuilder {
        <PostV1ReportsCostCenterItemsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsCostCenterItemsResponseBuilder {
    from_date: Option<String>,
    to_date: Option<String>,
    rows: Option<Vec<PostV1ReportsCostCenterItemsResponseRowsItem>>,
}

impl PostV1ReportsCostCenterItemsResponseBuilder {
    pub fn from_date(mut self, value: impl Into<String>) -> Self {
        self.from_date = Some(value.into());
        self
    }

    pub fn to_date(mut self, value: impl Into<String>) -> Self {
        self.to_date = Some(value.into());
        self
    }

    pub fn rows(mut self, value: Vec<PostV1ReportsCostCenterItemsResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsCostCenterItemsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_date`](PostV1ReportsCostCenterItemsResponseBuilder::from_date)
    /// - [`to_date`](PostV1ReportsCostCenterItemsResponseBuilder::to_date)
    /// - [`rows`](PostV1ReportsCostCenterItemsResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1ReportsCostCenterItemsResponse, BuildError> {
        Ok(PostV1ReportsCostCenterItemsResponse {
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
