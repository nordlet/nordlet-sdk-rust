pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsCostCenterActivityResponse {
    #[serde(rename = "costCenter")]
    #[serde(default)]
    pub cost_center: PostV1ReportsCostCenterActivityResponseCostCenter,
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: String,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: String,
    #[serde(default)]
    pub rows: Vec<PostV1ReportsCostCenterActivityResponseRowsItem>,
}

impl PostV1ReportsCostCenterActivityResponse {
    pub fn builder() -> PostV1ReportsCostCenterActivityResponseBuilder {
        <PostV1ReportsCostCenterActivityResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsCostCenterActivityResponseBuilder {
    cost_center: Option<PostV1ReportsCostCenterActivityResponseCostCenter>,
    from_date: Option<String>,
    to_date: Option<String>,
    rows: Option<Vec<PostV1ReportsCostCenterActivityResponseRowsItem>>,
}

impl PostV1ReportsCostCenterActivityResponseBuilder {
    pub fn cost_center(mut self, value: PostV1ReportsCostCenterActivityResponseCostCenter) -> Self {
        self.cost_center = Some(value);
        self
    }

    pub fn from_date(mut self, value: impl Into<String>) -> Self {
        self.from_date = Some(value.into());
        self
    }

    pub fn to_date(mut self, value: impl Into<String>) -> Self {
        self.to_date = Some(value.into());
        self
    }

    pub fn rows(mut self, value: Vec<PostV1ReportsCostCenterActivityResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsCostCenterActivityResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`cost_center`](PostV1ReportsCostCenterActivityResponseBuilder::cost_center)
    /// - [`from_date`](PostV1ReportsCostCenterActivityResponseBuilder::from_date)
    /// - [`to_date`](PostV1ReportsCostCenterActivityResponseBuilder::to_date)
    /// - [`rows`](PostV1ReportsCostCenterActivityResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1ReportsCostCenterActivityResponse, BuildError> {
        Ok(PostV1ReportsCostCenterActivityResponse {
            cost_center: self
                .cost_center
                .ok_or_else(|| BuildError::missing_field("cost_center"))?,
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
