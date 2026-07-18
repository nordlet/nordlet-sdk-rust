pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsEuSmeCrossBorderReportComputeResponse {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub quarter: i64,
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: String,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: String,
    #[serde(default)]
    pub currency: String,
    #[serde(default)]
    pub rows: Vec<PostV1DeclarationsEuSmeCrossBorderReportComputeResponseRowsItem>,
    #[serde(default)]
    pub total: String,
    #[serde(default)]
    pub warnings: Vec<String>,
}

impl PostV1DeclarationsEuSmeCrossBorderReportComputeResponse {
    pub fn builder() -> PostV1DeclarationsEuSmeCrossBorderReportComputeResponseBuilder {
        <PostV1DeclarationsEuSmeCrossBorderReportComputeResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsEuSmeCrossBorderReportComputeResponseBuilder {
    year: Option<i64>,
    quarter: Option<i64>,
    from_date: Option<String>,
    to_date: Option<String>,
    currency: Option<String>,
    rows: Option<Vec<PostV1DeclarationsEuSmeCrossBorderReportComputeResponseRowsItem>>,
    total: Option<String>,
    warnings: Option<Vec<String>>,
}

impl PostV1DeclarationsEuSmeCrossBorderReportComputeResponseBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn quarter(mut self, value: i64) -> Self {
        self.quarter = Some(value);
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

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn rows(
        mut self,
        value: Vec<PostV1DeclarationsEuSmeCrossBorderReportComputeResponseRowsItem>,
    ) -> Self {
        self.rows = Some(value);
        self
    }

    pub fn total(mut self, value: impl Into<String>) -> Self {
        self.total = Some(value.into());
        self
    }

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsEuSmeCrossBorderReportComputeResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsEuSmeCrossBorderReportComputeResponseBuilder::year)
    /// - [`quarter`](PostV1DeclarationsEuSmeCrossBorderReportComputeResponseBuilder::quarter)
    /// - [`from_date`](PostV1DeclarationsEuSmeCrossBorderReportComputeResponseBuilder::from_date)
    /// - [`to_date`](PostV1DeclarationsEuSmeCrossBorderReportComputeResponseBuilder::to_date)
    /// - [`currency`](PostV1DeclarationsEuSmeCrossBorderReportComputeResponseBuilder::currency)
    /// - [`rows`](PostV1DeclarationsEuSmeCrossBorderReportComputeResponseBuilder::rows)
    /// - [`total`](PostV1DeclarationsEuSmeCrossBorderReportComputeResponseBuilder::total)
    /// - [`warnings`](PostV1DeclarationsEuSmeCrossBorderReportComputeResponseBuilder::warnings)
    pub fn build(
        self,
    ) -> Result<PostV1DeclarationsEuSmeCrossBorderReportComputeResponse, BuildError> {
        Ok(PostV1DeclarationsEuSmeCrossBorderReportComputeResponse {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            quarter: self
                .quarter
                .ok_or_else(|| BuildError::missing_field("quarter"))?,
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
        })
    }
}
