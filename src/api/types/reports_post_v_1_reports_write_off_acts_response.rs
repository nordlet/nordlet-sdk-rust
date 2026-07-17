pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsWriteOffActsResponse {
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: String,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: String,
    #[serde(default)]
    pub rows: Vec<PostV1ReportsWriteOffActsResponseRowsItem>,
    #[serde(rename = "totalCost")]
    #[serde(default)]
    pub total_cost: String,
}

impl PostV1ReportsWriteOffActsResponse {
    pub fn builder() -> PostV1ReportsWriteOffActsResponseBuilder {
        <PostV1ReportsWriteOffActsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsWriteOffActsResponseBuilder {
    from_date: Option<String>,
    to_date: Option<String>,
    rows: Option<Vec<PostV1ReportsWriteOffActsResponseRowsItem>>,
    total_cost: Option<String>,
}

impl PostV1ReportsWriteOffActsResponseBuilder {
    pub fn from_date(mut self, value: impl Into<String>) -> Self {
        self.from_date = Some(value.into());
        self
    }

    pub fn to_date(mut self, value: impl Into<String>) -> Self {
        self.to_date = Some(value.into());
        self
    }

    pub fn rows(mut self, value: Vec<PostV1ReportsWriteOffActsResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    pub fn total_cost(mut self, value: impl Into<String>) -> Self {
        self.total_cost = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsWriteOffActsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_date`](PostV1ReportsWriteOffActsResponseBuilder::from_date)
    /// - [`to_date`](PostV1ReportsWriteOffActsResponseBuilder::to_date)
    /// - [`rows`](PostV1ReportsWriteOffActsResponseBuilder::rows)
    /// - [`total_cost`](PostV1ReportsWriteOffActsResponseBuilder::total_cost)
    pub fn build(self) -> Result<PostV1ReportsWriteOffActsResponse, BuildError> {
        Ok(PostV1ReportsWriteOffActsResponse {
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
            total_cost: self
                .total_cost
                .ok_or_else(|| BuildError::missing_field("total_cost"))?,
        })
    }
}
