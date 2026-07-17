pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLtSamComputeResponse {
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
    pub persons: Vec<PostV1DeclarationsLtSamComputeResponsePersonsItem>,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
}

impl PostV1DeclarationsLtSamComputeResponse {
    pub fn builder() -> PostV1DeclarationsLtSamComputeResponseBuilder {
        <PostV1DeclarationsLtSamComputeResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtSamComputeResponseBuilder {
    year: Option<i64>,
    month: Option<i64>,
    insured_count: Option<i64>,
    insured_income_total: Option<String>,
    contributions_total: Option<String>,
    persons: Option<Vec<PostV1DeclarationsLtSamComputeResponsePersonsItem>>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
}

impl PostV1DeclarationsLtSamComputeResponseBuilder {
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

    pub fn persons(
        mut self,
        value: Vec<PostV1DeclarationsLtSamComputeResponsePersonsItem>,
    ) -> Self {
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtSamComputeResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsLtSamComputeResponseBuilder::year)
    /// - [`month`](PostV1DeclarationsLtSamComputeResponseBuilder::month)
    /// - [`insured_count`](PostV1DeclarationsLtSamComputeResponseBuilder::insured_count)
    /// - [`insured_income_total`](PostV1DeclarationsLtSamComputeResponseBuilder::insured_income_total)
    /// - [`contributions_total`](PostV1DeclarationsLtSamComputeResponseBuilder::contributions_total)
    /// - [`persons`](PostV1DeclarationsLtSamComputeResponseBuilder::persons)
    /// - [`warnings`](PostV1DeclarationsLtSamComputeResponseBuilder::warnings)
    /// - [`notes`](PostV1DeclarationsLtSamComputeResponseBuilder::notes)
    pub fn build(self) -> Result<PostV1DeclarationsLtSamComputeResponse, BuildError> {
        Ok(PostV1DeclarationsLtSamComputeResponse {
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
