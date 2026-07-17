pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsEuPurchasesResponse {
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: String,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: String,
    #[serde(default)]
    pub rows: Vec<PostV1ReportsEuPurchasesResponseRowsItem>,
    #[serde(default)]
    pub totals: PostV1ReportsEuPurchasesResponseTotals,
}

impl PostV1ReportsEuPurchasesResponse {
    pub fn builder() -> PostV1ReportsEuPurchasesResponseBuilder {
        <PostV1ReportsEuPurchasesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsEuPurchasesResponseBuilder {
    from_date: Option<String>,
    to_date: Option<String>,
    rows: Option<Vec<PostV1ReportsEuPurchasesResponseRowsItem>>,
    totals: Option<PostV1ReportsEuPurchasesResponseTotals>,
}

impl PostV1ReportsEuPurchasesResponseBuilder {
    pub fn from_date(mut self, value: impl Into<String>) -> Self {
        self.from_date = Some(value.into());
        self
    }

    pub fn to_date(mut self, value: impl Into<String>) -> Self {
        self.to_date = Some(value.into());
        self
    }

    pub fn rows(mut self, value: Vec<PostV1ReportsEuPurchasesResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    pub fn totals(mut self, value: PostV1ReportsEuPurchasesResponseTotals) -> Self {
        self.totals = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsEuPurchasesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_date`](PostV1ReportsEuPurchasesResponseBuilder::from_date)
    /// - [`to_date`](PostV1ReportsEuPurchasesResponseBuilder::to_date)
    /// - [`rows`](PostV1ReportsEuPurchasesResponseBuilder::rows)
    /// - [`totals`](PostV1ReportsEuPurchasesResponseBuilder::totals)
    pub fn build(self) -> Result<PostV1ReportsEuPurchasesResponse, BuildError> {
        Ok(PostV1ReportsEuPurchasesResponse {
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
