pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtSamComputeDeclarationsResponse {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
    #[serde(rename = "insuredCount")]
    #[serde(default)]
    pub insured_count: i64,
    #[serde(rename = "insuredIncomeTotal")]
    #[serde(default)]
    pub insured_income_total: String,
    #[serde(rename = "contributionsTotal")]
    #[serde(default)]
    pub contributions_total: String,
    #[serde(default)]
    pub persons: Vec<LtSamComputeDeclarationsResponsePersonsItem>,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
}

impl LtSamComputeDeclarationsResponse {
    pub fn builder() -> LtSamComputeDeclarationsResponseBuilder {
        <LtSamComputeDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtSamComputeDeclarationsResponseBuilder {
    year: Option<i64>,
    month: Option<i64>,
    insured_count: Option<i64>,
    insured_income_total: Option<String>,
    contributions_total: Option<String>,
    persons: Option<Vec<LtSamComputeDeclarationsResponsePersonsItem>>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
}

impl LtSamComputeDeclarationsResponseBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    pub fn insured_count(mut self, value: i64) -> Self {
        self.insured_count = Some(value);
        self
    }

    pub fn insured_income_total(mut self, value: impl Into<String>) -> Self {
        self.insured_income_total = Some(value.into());
        self
    }

    pub fn contributions_total(mut self, value: impl Into<String>) -> Self {
        self.contributions_total = Some(value.into());
        self
    }

    pub fn persons(mut self, value: Vec<LtSamComputeDeclarationsResponsePersonsItem>) -> Self {
        self.persons = Some(value);
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

    /// Consumes the builder and constructs a [`LtSamComputeDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](LtSamComputeDeclarationsResponseBuilder::year)
    /// - [`month`](LtSamComputeDeclarationsResponseBuilder::month)
    /// - [`insured_count`](LtSamComputeDeclarationsResponseBuilder::insured_count)
    /// - [`insured_income_total`](LtSamComputeDeclarationsResponseBuilder::insured_income_total)
    /// - [`contributions_total`](LtSamComputeDeclarationsResponseBuilder::contributions_total)
    /// - [`persons`](LtSamComputeDeclarationsResponseBuilder::persons)
    /// - [`warnings`](LtSamComputeDeclarationsResponseBuilder::warnings)
    /// - [`notes`](LtSamComputeDeclarationsResponseBuilder::notes)
    pub fn build(self) -> Result<LtSamComputeDeclarationsResponse, BuildError> {
        Ok(LtSamComputeDeclarationsResponse {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
            insured_count: self
                .insured_count
                .ok_or_else(|| BuildError::missing_field("insured_count"))?,
            insured_income_total: self
                .insured_income_total
                .ok_or_else(|| BuildError::missing_field("insured_income_total"))?,
            contributions_total: self
                .contributions_total
                .ok_or_else(|| BuildError::missing_field("contributions_total"))?,
            persons: self
                .persons
                .ok_or_else(|| BuildError::missing_field("persons"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
            notes: self
                .notes
                .ok_or_else(|| BuildError::missing_field("notes"))?,
        })
    }
}
