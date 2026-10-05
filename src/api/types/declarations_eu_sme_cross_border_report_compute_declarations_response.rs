pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EuSmeCrossBorderReportComputeDeclarationsResponse {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub quarter: i64,
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: NaiveDate,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: NaiveDate,
    #[serde(default)]
    pub currency: String,
    #[serde(default)]
    pub rows: Vec<EuSmeCrossBorderReportComputeDeclarationsResponseRowsItem>,
    #[serde(default)]
    pub total: String,
    #[serde(default)]
    pub warnings: Vec<String>,
}

impl EuSmeCrossBorderReportComputeDeclarationsResponse {
    pub fn builder() -> EuSmeCrossBorderReportComputeDeclarationsResponseBuilder {
        <EuSmeCrossBorderReportComputeDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EuSmeCrossBorderReportComputeDeclarationsResponseBuilder {
    year: Option<i64>,
    quarter: Option<i64>,
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
    currency: Option<String>,
    rows: Option<Vec<EuSmeCrossBorderReportComputeDeclarationsResponseRowsItem>>,
    total: Option<String>,
    warnings: Option<Vec<String>>,
}

impl EuSmeCrossBorderReportComputeDeclarationsResponseBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn quarter(mut self, value: i64) -> Self {
        self.quarter = Some(value);
        self
    }

    pub fn from_date(mut self, value: NaiveDate) -> Self {
        self.from_date = Some(value);
        self
    }

    pub fn to_date(mut self, value: NaiveDate) -> Self {
        self.to_date = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn rows(
        mut self,
        value: Vec<EuSmeCrossBorderReportComputeDeclarationsResponseRowsItem>,
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

    /// Consumes the builder and constructs a [`EuSmeCrossBorderReportComputeDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](EuSmeCrossBorderReportComputeDeclarationsResponseBuilder::year)
    /// - [`quarter`](EuSmeCrossBorderReportComputeDeclarationsResponseBuilder::quarter)
    /// - [`from_date`](EuSmeCrossBorderReportComputeDeclarationsResponseBuilder::from_date)
    /// - [`to_date`](EuSmeCrossBorderReportComputeDeclarationsResponseBuilder::to_date)
    /// - [`currency`](EuSmeCrossBorderReportComputeDeclarationsResponseBuilder::currency)
    /// - [`rows`](EuSmeCrossBorderReportComputeDeclarationsResponseBuilder::rows)
    /// - [`total`](EuSmeCrossBorderReportComputeDeclarationsResponseBuilder::total)
    /// - [`warnings`](EuSmeCrossBorderReportComputeDeclarationsResponseBuilder::warnings)
    pub fn build(self) -> Result<EuSmeCrossBorderReportComputeDeclarationsResponse, BuildError> {
        Ok(EuSmeCrossBorderReportComputeDeclarationsResponse {
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
