pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostV1PayrollCalcRequest {
    #[serde(rename = "taxableBase")]
    #[serde(default)]
    pub taxable_base: String,
    #[serde(default)]
    pub date: String,
    #[serde(rename = "applyAllowance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_allowance: Option<bool>,
    #[serde(rename = "allowanceOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowance_override: Option<String>,
    #[serde(rename = "pensionAccumulation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pension_accumulation: Option<bool>,
    #[serde(rename = "fixedTerm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_term: Option<bool>,
    #[serde(rename = "benefitInKind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub benefit_in_kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<HashMap<String, String>>,
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
    apply_allowance: Option<bool>,
    allowance_override: Option<String>,
    pension_accumulation: Option<bool>,
    fixed_term: Option<bool>,
    benefit_in_kind: Option<String>,
    options: Option<HashMap<String, String>>,
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

    pub fn apply_allowance(mut self, value: bool) -> Self {
        self.apply_allowance = Some(value);
        self
    }

    pub fn allowance_override(mut self, value: impl Into<String>) -> Self {
        self.allowance_override = Some(value.into());
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

    pub fn benefit_in_kind(mut self, value: impl Into<String>) -> Self {
        self.benefit_in_kind = Some(value.into());
        self
    }

    pub fn options(mut self, value: HashMap<String, String>) -> Self {
        self.options = Some(value);
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
            apply_allowance: self.apply_allowance,
            allowance_override: self.allowance_override,
            pension_accumulation: self.pension_accumulation,
            fixed_term: self.fixed_term,
            benefit_in_kind: self.benefit_in_kind,
            options: self.options,
        })
    }
}
