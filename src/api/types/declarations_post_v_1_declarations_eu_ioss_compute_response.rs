pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsEuIossComputeResponse {
    #[serde(rename = "periodYear")]
    #[serde(default)]
    pub period_year: i64,
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: String,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: String,
    #[serde(rename = "memberStateOfIdentification")]
    #[serde(default)]
    pub member_state_of_identification: String,
    #[serde(default)]
    pub rows: Vec<PostV1DeclarationsEuIossComputeResponseRowsItem>,
    #[serde(default)]
    pub totals: PostV1DeclarationsEuIossComputeResponseTotals,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(rename = "periodMonth")]
    #[serde(default)]
    pub period_month: i64,
}

impl PostV1DeclarationsEuIossComputeResponse {
    pub fn builder() -> PostV1DeclarationsEuIossComputeResponseBuilder {
        <PostV1DeclarationsEuIossComputeResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsEuIossComputeResponseBuilder {
    period_year: Option<i64>,
    from_date: Option<String>,
    to_date: Option<String>,
    member_state_of_identification: Option<String>,
    rows: Option<Vec<PostV1DeclarationsEuIossComputeResponseRowsItem>>,
    totals: Option<PostV1DeclarationsEuIossComputeResponseTotals>,
    warnings: Option<Vec<String>>,
    period_month: Option<i64>,
}

impl PostV1DeclarationsEuIossComputeResponseBuilder {
    pub fn period_year(mut self, value: i64) -> Self {
        self.period_year = Some(value);
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

    pub fn member_state_of_identification(mut self, value: impl Into<String>) -> Self {
        self.member_state_of_identification = Some(value.into());
        self
    }

    pub fn rows(mut self, value: Vec<PostV1DeclarationsEuIossComputeResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    pub fn totals(mut self, value: PostV1DeclarationsEuIossComputeResponseTotals) -> Self {
        self.totals = Some(value);
        self
    }

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    pub fn period_month(mut self, value: i64) -> Self {
        self.period_month = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsEuIossComputeResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`period_year`](PostV1DeclarationsEuIossComputeResponseBuilder::period_year)
    /// - [`from_date`](PostV1DeclarationsEuIossComputeResponseBuilder::from_date)
    /// - [`to_date`](PostV1DeclarationsEuIossComputeResponseBuilder::to_date)
    /// - [`member_state_of_identification`](PostV1DeclarationsEuIossComputeResponseBuilder::member_state_of_identification)
    /// - [`rows`](PostV1DeclarationsEuIossComputeResponseBuilder::rows)
    /// - [`totals`](PostV1DeclarationsEuIossComputeResponseBuilder::totals)
    /// - [`warnings`](PostV1DeclarationsEuIossComputeResponseBuilder::warnings)
    /// - [`period_month`](PostV1DeclarationsEuIossComputeResponseBuilder::period_month)
    pub fn build(self) -> Result<PostV1DeclarationsEuIossComputeResponse, BuildError> {
        Ok(PostV1DeclarationsEuIossComputeResponse {
            period_year: self
                .period_year
                .ok_or_else(|| BuildError::missing_field("period_year"))?,
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
            member_state_of_identification: self
                .member_state_of_identification
                .ok_or_else(|| BuildError::missing_field("member_state_of_identification"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
            totals: self
                .totals
                .ok_or_else(|| BuildError::missing_field("totals"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
            period_month: self
                .period_month
                .ok_or_else(|| BuildError::missing_field("period_month"))?,
        })
    }
}
