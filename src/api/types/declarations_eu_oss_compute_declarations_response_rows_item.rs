pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EuOssComputeDeclarationsResponseRowsItem {
    #[serde(rename = "countryCode")]
    #[serde(default)]
    pub country_code: String,
    #[serde(rename = "rateType")]
    pub rate_type: EuOssComputeDeclarationsResponseRowsItemRateType,
    #[serde(rename = "vatRatePercent")]
    #[serde(default)]
    pub vat_rate_percent: String,
    #[serde(rename = "taxableAmount")]
    #[serde(default)]
    pub taxable_amount: String,
    #[serde(rename = "vatAmount")]
    #[serde(default)]
    pub vat_amount: String,
    #[serde(default)]
    pub documents: i64,
}

impl EuOssComputeDeclarationsResponseRowsItem {
    pub fn builder() -> EuOssComputeDeclarationsResponseRowsItemBuilder {
        <EuOssComputeDeclarationsResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EuOssComputeDeclarationsResponseRowsItemBuilder {
    country_code: Option<String>,
    rate_type: Option<EuOssComputeDeclarationsResponseRowsItemRateType>,
    vat_rate_percent: Option<String>,
    taxable_amount: Option<String>,
    vat_amount: Option<String>,
    documents: Option<i64>,
}

impl EuOssComputeDeclarationsResponseRowsItemBuilder {
    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn rate_type(mut self, value: EuOssComputeDeclarationsResponseRowsItemRateType) -> Self {
        self.rate_type = Some(value);
        self
    }

    pub fn vat_rate_percent(mut self, value: impl Into<String>) -> Self {
        self.vat_rate_percent = Some(value.into());
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

    /// Consumes the builder and constructs a [`EuOssComputeDeclarationsResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`country_code`](EuOssComputeDeclarationsResponseRowsItemBuilder::country_code)
    /// - [`rate_type`](EuOssComputeDeclarationsResponseRowsItemBuilder::rate_type)
    /// - [`vat_rate_percent`](EuOssComputeDeclarationsResponseRowsItemBuilder::vat_rate_percent)
    /// - [`taxable_amount`](EuOssComputeDeclarationsResponseRowsItemBuilder::taxable_amount)
    /// - [`vat_amount`](EuOssComputeDeclarationsResponseRowsItemBuilder::vat_amount)
    /// - [`documents`](EuOssComputeDeclarationsResponseRowsItemBuilder::documents)
    pub fn build(self) -> Result<EuOssComputeDeclarationsResponseRowsItem, BuildError> {
        Ok(EuOssComputeDeclarationsResponseRowsItem {
            country_code: self
                .country_code
                .ok_or_else(|| BuildError::missing_field("country_code"))?,
            rate_type: self
                .rate_type
                .ok_or_else(|| BuildError::missing_field("rate_type"))?,
            vat_rate_percent: self
                .vat_rate_percent
                .ok_or_else(|| BuildError::missing_field("vat_rate_percent"))?,
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
