pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsPosSalesResponse {
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: String,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: String,
    #[serde(default)]
    pub rows: Vec<PostV1ReportsPosSalesResponseRowsItem>,
    #[serde(rename = "byRate")]
    #[serde(default)]
    pub by_rate: Vec<PostV1ReportsPosSalesResponseByRateItem>,
    #[serde(default)]
    pub totals: PostV1ReportsPosSalesResponseTotals,
}

impl PostV1ReportsPosSalesResponse {
    pub fn builder() -> PostV1ReportsPosSalesResponseBuilder {
        <PostV1ReportsPosSalesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsPosSalesResponseBuilder {
    from_date: Option<String>,
    to_date: Option<String>,
    rows: Option<Vec<PostV1ReportsPosSalesResponseRowsItem>>,
    by_rate: Option<Vec<PostV1ReportsPosSalesResponseByRateItem>>,
    totals: Option<PostV1ReportsPosSalesResponseTotals>,
}

impl PostV1ReportsPosSalesResponseBuilder {
    pub fn from_date(mut self, value: impl Into<String>) -> Self {
        self.from_date = Some(value.into());
        self
    }

    pub fn to_date(mut self, value: impl Into<String>) -> Self {
        self.to_date = Some(value.into());
        self
    }

    pub fn rows(mut self, value: Vec<PostV1ReportsPosSalesResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    pub fn by_rate(mut self, value: Vec<PostV1ReportsPosSalesResponseByRateItem>) -> Self {
        self.by_rate = Some(value);
        self
    }

    pub fn totals(mut self, value: PostV1ReportsPosSalesResponseTotals) -> Self {
        self.totals = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsPosSalesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_date`](PostV1ReportsPosSalesResponseBuilder::from_date)
    /// - [`to_date`](PostV1ReportsPosSalesResponseBuilder::to_date)
    /// - [`rows`](PostV1ReportsPosSalesResponseBuilder::rows)
    /// - [`by_rate`](PostV1ReportsPosSalesResponseBuilder::by_rate)
    /// - [`totals`](PostV1ReportsPosSalesResponseBuilder::totals)
    pub fn build(self) -> Result<PostV1ReportsPosSalesResponse, BuildError> {
        Ok(PostV1ReportsPosSalesResponse {
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
            by_rate: self
                .by_rate
                .ok_or_else(|| BuildError::missing_field("by_rate"))?,
            totals: self
                .totals
                .ok_or_else(|| BuildError::missing_field("totals"))?,
        })
    }
}
