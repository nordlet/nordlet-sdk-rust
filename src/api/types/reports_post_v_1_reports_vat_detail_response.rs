pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsVatDetailResponse {
    #[serde(default)]
    pub side: String,
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: String,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: String,
    #[serde(default)]
    pub rows: Vec<PostV1ReportsVatDetailResponseRowsItem>,
    #[serde(default)]
    pub totals: PostV1ReportsVatDetailResponseTotals,
}

impl PostV1ReportsVatDetailResponse {
    pub fn builder() -> PostV1ReportsVatDetailResponseBuilder {
        <PostV1ReportsVatDetailResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsVatDetailResponseBuilder {
    side: Option<String>,
    from_date: Option<String>,
    to_date: Option<String>,
    rows: Option<Vec<PostV1ReportsVatDetailResponseRowsItem>>,
    totals: Option<PostV1ReportsVatDetailResponseTotals>,
}

impl PostV1ReportsVatDetailResponseBuilder {
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

    pub fn rows(mut self, value: Vec<PostV1ReportsVatDetailResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    pub fn totals(mut self, value: PostV1ReportsVatDetailResponseTotals) -> Self {
        self.totals = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsVatDetailResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`side`](PostV1ReportsVatDetailResponseBuilder::side)
    /// - [`from_date`](PostV1ReportsVatDetailResponseBuilder::from_date)
    /// - [`to_date`](PostV1ReportsVatDetailResponseBuilder::to_date)
    /// - [`rows`](PostV1ReportsVatDetailResponseBuilder::rows)
    /// - [`totals`](PostV1ReportsVatDetailResponseBuilder::totals)
    pub fn build(self) -> Result<PostV1ReportsVatDetailResponse, BuildError> {
        Ok(PostV1ReportsVatDetailResponse {
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
