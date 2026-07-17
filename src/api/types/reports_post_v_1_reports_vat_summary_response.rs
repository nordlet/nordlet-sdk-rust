pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsVatSummaryResponse {
    #[serde(default)]
    pub side: String,
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: String,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: String,
    #[serde(default)]
    pub rows: Vec<PostV1ReportsVatSummaryResponseRowsItem>,
    #[serde(default)]
    pub totals: PostV1ReportsVatSummaryResponseTotals,
}

impl PostV1ReportsVatSummaryResponse {
    pub fn builder() -> PostV1ReportsVatSummaryResponseBuilder {
        <PostV1ReportsVatSummaryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsVatSummaryResponseBuilder {
    side: Option<String>,
    from_date: Option<String>,
    to_date: Option<String>,
    rows: Option<Vec<PostV1ReportsVatSummaryResponseRowsItem>>,
    totals: Option<PostV1ReportsVatSummaryResponseTotals>,
}

impl PostV1ReportsVatSummaryResponseBuilder {
    pub fn side(mut self, value: impl Into<String>) -> Self {
        self.side = Some(value.into());
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

    pub fn rows(mut self, value: Vec<PostV1ReportsVatSummaryResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    pub fn totals(mut self, value: PostV1ReportsVatSummaryResponseTotals) -> Self {
        self.totals = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsVatSummaryResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`side`](PostV1ReportsVatSummaryResponseBuilder::side)
    /// - [`from_date`](PostV1ReportsVatSummaryResponseBuilder::from_date)
    /// - [`to_date`](PostV1ReportsVatSummaryResponseBuilder::to_date)
    /// - [`rows`](PostV1ReportsVatSummaryResponseBuilder::rows)
    /// - [`totals`](PostV1ReportsVatSummaryResponseBuilder::totals)
    pub fn build(self) -> Result<PostV1ReportsVatSummaryResponse, BuildError> {
        Ok(PostV1ReportsVatSummaryResponse {
            side: self.side.ok_or_else(|| BuildError::missing_field("side"))?,
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
            totals: self
                .totals
                .ok_or_else(|| BuildError::missing_field("totals"))?,
        })
    }
}
