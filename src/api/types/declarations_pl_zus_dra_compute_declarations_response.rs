pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PlZusDraComputeDeclarationsResponse {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
    #[serde(default)]
    pub source: String,
    #[serde(rename = "runStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_status: Option<String>,
    #[serde(rename = "insuredCount")]
    #[serde(default)]
    pub insured_count: i64,
    #[serde(default)]
    pub rows: Vec<PlZusDraComputeDeclarationsResponseRowsItem>,
    #[serde(rename = "socialTotal")]
    #[serde(default)]
    pub social_total: String,
    #[serde(rename = "healthTotal")]
    #[serde(default)]
    pub health_total: String,
    #[serde(rename = "fundsTotal")]
    #[serde(default)]
    pub funds_total: String,
    #[serde(default)]
    pub total: String,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
}

impl PlZusDraComputeDeclarationsResponse {
    pub fn builder() -> PlZusDraComputeDeclarationsResponseBuilder {
        <PlZusDraComputeDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlZusDraComputeDeclarationsResponseBuilder {
    year: Option<i64>,
    month: Option<i64>,
    source: Option<String>,
    run_status: Option<String>,
    insured_count: Option<i64>,
    rows: Option<Vec<PlZusDraComputeDeclarationsResponseRowsItem>>,
    social_total: Option<String>,
    health_total: Option<String>,
    funds_total: Option<String>,
    total: Option<String>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
}

impl PlZusDraComputeDeclarationsResponseBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn run_status(mut self, value: impl Into<String>) -> Self {
        self.run_status = Some(value.into());
        self
    }

    pub fn insured_count(mut self, value: i64) -> Self {
        self.insured_count = Some(value);
        self
    }

    pub fn rows(mut self, value: Vec<PlZusDraComputeDeclarationsResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    pub fn social_total(mut self, value: impl Into<String>) -> Self {
        self.social_total = Some(value.into());
        self
    }

    pub fn health_total(mut self, value: impl Into<String>) -> Self {
        self.health_total = Some(value.into());
        self
    }

    pub fn funds_total(mut self, value: impl Into<String>) -> Self {
        self.funds_total = Some(value.into());
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

    pub fn notes(mut self, value: Vec<String>) -> Self {
        self.notes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PlZusDraComputeDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PlZusDraComputeDeclarationsResponseBuilder::year)
    /// - [`month`](PlZusDraComputeDeclarationsResponseBuilder::month)
    /// - [`source`](PlZusDraComputeDeclarationsResponseBuilder::source)
    /// - [`insured_count`](PlZusDraComputeDeclarationsResponseBuilder::insured_count)
    /// - [`rows`](PlZusDraComputeDeclarationsResponseBuilder::rows)
    /// - [`social_total`](PlZusDraComputeDeclarationsResponseBuilder::social_total)
    /// - [`health_total`](PlZusDraComputeDeclarationsResponseBuilder::health_total)
    /// - [`funds_total`](PlZusDraComputeDeclarationsResponseBuilder::funds_total)
    /// - [`total`](PlZusDraComputeDeclarationsResponseBuilder::total)
    /// - [`warnings`](PlZusDraComputeDeclarationsResponseBuilder::warnings)
    /// - [`notes`](PlZusDraComputeDeclarationsResponseBuilder::notes)
    pub fn build(self) -> Result<PlZusDraComputeDeclarationsResponse, BuildError> {
        Ok(PlZusDraComputeDeclarationsResponse {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
            run_status: self.run_status,
            insured_count: self
                .insured_count
                .ok_or_else(|| BuildError::missing_field("insured_count"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
            social_total: self
                .social_total
                .ok_or_else(|| BuildError::missing_field("social_total"))?,
            health_total: self
                .health_total
                .ok_or_else(|| BuildError::missing_field("health_total"))?,
            funds_total: self
                .funds_total
                .ok_or_else(|| BuildError::missing_field("funds_total"))?,
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
            notes: self
                .notes
                .ok_or_else(|| BuildError::missing_field("notes"))?,
        })
    }
}
