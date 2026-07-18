pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsEuIossComputeResponseCorrectionsItem {
    #[serde(rename = "countryCode")]
    #[serde(default)]
    pub country_code: String,
    #[serde(rename = "periodYear")]
    #[serde(default)]
    pub period_year: i64,
    #[serde(rename = "periodQuarter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_quarter: Option<i64>,
    #[serde(rename = "periodMonth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_month: Option<i64>,
    #[serde(rename = "taxableAmount")]
    #[serde(default)]
    pub taxable_amount: String,
    #[serde(rename = "vatAmount")]
    #[serde(default)]
    pub vat_amount: String,
    #[serde(default)]
    pub documents: i64,
}

impl PostV1DeclarationsEuIossComputeResponseCorrectionsItem {
    pub fn builder() -> PostV1DeclarationsEuIossComputeResponseCorrectionsItemBuilder {
        <PostV1DeclarationsEuIossComputeResponseCorrectionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsEuIossComputeResponseCorrectionsItemBuilder {
    country_code: Option<String>,
    period_year: Option<i64>,
    period_quarter: Option<i64>,
    period_month: Option<i64>,
    taxable_amount: Option<String>,
    vat_amount: Option<String>,
    documents: Option<i64>,
}

impl PostV1DeclarationsEuIossComputeResponseCorrectionsItemBuilder {
    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn period_year(mut self, value: i64) -> Self {
        self.period_year = Some(value);
        self
    }

    pub fn period_quarter(mut self, value: i64) -> Self {
        self.period_quarter = Some(value);
        self
    }

    pub fn period_month(mut self, value: i64) -> Self {
        self.period_month = Some(value);
        self
    }

    pub fn taxable_amount(mut self, value: impl Into<String>) -> Self {
        self.taxable_amount = Some(value.into());
        self
    }

    pub fn vat_amount(mut self, value: impl Into<String>) -> Self {
        self.vat_amount = Some(value.into());
        self
    }

    pub fn documents(mut self, value: i64) -> Self {
        self.documents = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsEuIossComputeResponseCorrectionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`country_code`](PostV1DeclarationsEuIossComputeResponseCorrectionsItemBuilder::country_code)
    /// - [`period_year`](PostV1DeclarationsEuIossComputeResponseCorrectionsItemBuilder::period_year)
    /// - [`taxable_amount`](PostV1DeclarationsEuIossComputeResponseCorrectionsItemBuilder::taxable_amount)
    /// - [`vat_amount`](PostV1DeclarationsEuIossComputeResponseCorrectionsItemBuilder::vat_amount)
    /// - [`documents`](PostV1DeclarationsEuIossComputeResponseCorrectionsItemBuilder::documents)
    pub fn build(
        self,
    ) -> Result<PostV1DeclarationsEuIossComputeResponseCorrectionsItem, BuildError> {
        Ok(PostV1DeclarationsEuIossComputeResponseCorrectionsItem {
            country_code: self
                .country_code
                .ok_or_else(|| BuildError::missing_field("country_code"))?,
            period_year: self
                .period_year
                .ok_or_else(|| BuildError::missing_field("period_year"))?,
            period_quarter: self.period_quarter,
            period_month: self.period_month,
            taxable_amount: self
                .taxable_amount
                .ok_or_else(|| BuildError::missing_field("taxable_amount"))?,
            vat_amount: self
                .vat_amount
                .ok_or_else(|| BuildError::missing_field("vat_amount"))?,
            documents: self
                .documents
                .ok_or_else(|| BuildError::missing_field("documents"))?,
        })
    }
}
