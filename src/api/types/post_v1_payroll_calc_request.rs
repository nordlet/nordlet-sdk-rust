pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PayrollCalcRequest {
    #[serde(rename = "taxableBase")]
    #[serde(default)]
    pub taxable_base: String,
    #[serde(default)]
    pub date: String,
    #[serde(rename = "applyNpd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_npd: Option<bool>,
    #[serde(rename = "npdOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub npd_override: Option<String>,
    #[serde(rename = "pensionAccumulation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pension_accumulation: Option<bool>,
    #[serde(rename = "fixedTerm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_term: Option<bool>,
}

impl PostV1PayrollCalcRequest {
    pub fn builder() -> PostV1PayrollCalcRequestBuilder {
        <PostV1PayrollCalcRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PayrollCalcRequestBuilder {
    taxable_base: Option<String>,
    date: Option<String>,
    apply_npd: Option<bool>,
    npd_override: Option<String>,
    pension_accumulation: Option<bool>,
    fixed_term: Option<bool>,
}

impl PostV1PayrollCalcRequestBuilder {
    pub fn taxable_base(mut self, value: impl Into<String>) -> Self {
        self.taxable_base = Some(value.into());
        self
    }

    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    pub fn apply_npd(mut self, value: bool) -> Self {
        self.apply_npd = Some(value);
        self
    }

    pub fn npd_override(mut self, value: impl Into<String>) -> Self {
        self.npd_override = Some(value.into());
        self
    }

    pub fn pension_accumulation(mut self, value: bool) -> Self {
        self.pension_accumulation = Some(value);
        self
    }

    pub fn fixed_term(mut self, value: bool) -> Self {
        self.fixed_term = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PayrollCalcRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`taxable_base`](PostV1PayrollCalcRequestBuilder::taxable_base)
    /// - [`date`](PostV1PayrollCalcRequestBuilder::date)
    pub fn build(self) -> Result<PostV1PayrollCalcRequest, BuildError> {
        Ok(PostV1PayrollCalcRequest {
            taxable_base: self
                .taxable_base
                .ok_or_else(|| BuildError::missing_field("taxable_base"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            apply_npd: self.apply_npd,
            npd_override: self.npd_override,
            pension_accumulation: self.pension_accumulation,
            fixed_term: self.fixed_term,
        })
    }
}
